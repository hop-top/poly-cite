use hop_top_cite::*;
use serde_json::{json, Value};
use std::{collections::HashMap, env, fs, path::PathBuf};

fn main() {
    let fixture_dir = PathBuf::from(
        env::args()
            .nth(1)
            .unwrap_or_else(|| "../spec/fixtures".to_string()),
    );
    let uri_fixture: Value = read_json(fixture_dir.join("cite-contract.json"));
    let handler_fixture: Value = read_json(fixture_dir.join("handler-contract.json"));
    let base_policy = policy_from_fixture(&uri_fixture, vec![], HashMap::new());

    let output = json!({
        "version": 1,
        "uri": {
            "valid": uri_fixture["validCases"].as_array().unwrap().iter().map(|tc| parse_case(tc, &base_policy)).collect::<Vec<_>>(),
            "invalid": uri_fixture["invalidCases"].as_array().unwrap().iter().map(|tc| invalid_parse_case(tc, &base_policy)).collect::<Vec<_>>(),
            "vanity": uri_fixture["vanityCases"].as_array().unwrap().iter().map(|tc| vanity_case(tc, &uri_fixture)).collect::<Vec<_>>(),
            "action": uri_fixture["actionCases"].as_array().unwrap().iter().map(|tc| action_case(tc, &uri_fixture)).collect::<Vec<_>>(),
        },
        "handler": {
            "valid": handler_fixture["cases"].as_array().unwrap().iter().map(handler_case).collect::<Vec<_>>(),
            "invalid": handler_fixture["invalidCases"].as_array().unwrap().iter().map(invalid_handler_case).collect::<Vec<_>>(),
        }
    });
    println!("{}", serde_json::to_string(&output).unwrap());
}

fn read_json(path: PathBuf) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn policy_from_fixture(
    fixture: &Value,
    vanity_aliases: Vec<VanityAlias>,
    action_routes: HashMap<String, ActionRoute>,
) -> Policy {
    Policy {
        default_namespace_segments: fixture["namespacePolicy"]["defaultNamespaceSegments"]
            .as_u64()
            .unwrap() as usize,
        scheme_namespace_segments: fixture["namespacePolicy"]["schemeNamespaceSegments"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_u64().unwrap() as usize))
            .collect(),
        vanity_aliases,
        action_routes,
    }
}

fn parse_case(tc: &Value, policy: &Policy) -> Value {
    let name = tc["name"].as_str().unwrap();
    let input = tc["input"].as_str().unwrap();
    parse_result(name, parse_with_policy(input, policy))
}

fn invalid_parse_case(tc: &Value, policy: &Policy) -> Value {
    let name = tc["name"].as_str().unwrap();
    let input = tc["input"].as_str().unwrap();
    json!({"name": name, "ok": parse_with_policy(input, policy).is_err()})
}

fn vanity_case(tc: &Value, fixture: &Value) -> Value {
    let aliases = tc["aliases"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|alias| VanityAlias {
            from: alias["from"].as_str().unwrap_or_default().to_string(),
            to: alias["to"].as_str().unwrap_or_default().to_string(),
            prefix: alias["prefix"].as_bool().unwrap_or(false),
            preserve_suffix: alias["preserveSuffix"].as_bool().unwrap_or(false),
        })
        .collect();
    let policy = policy_from_fixture(fixture, aliases, HashMap::new());
    let options = ParseOptions {
        strict: tc["options"]["strict"].as_bool().unwrap_or(false),
        json_ambiguity: tc["options"]["jsonAmbiguity"].as_bool().unwrap_or(false),
    };
    parse_result(
        tc["name"].as_str().unwrap(),
        parse_with_policy_options(tc["input"].as_str().unwrap(), &policy, options),
    )
}

fn action_case(tc: &Value, fixture: &Value) -> Value {
    let mut routes = HashMap::new();
    for (name, route) in tc["actionRoutes"].as_object().unwrap() {
        routes.insert(
            name.clone(),
            ActionRoute {
                command: route["command"].as_str().unwrap_or_default().to_string(),
                args: route["args"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|arg| arg.as_str().unwrap_or_default().to_string())
                    .collect(),
            },
        );
    }
    let policy = policy_from_fixture(fixture, vec![], routes);
    let name = tc["name"].as_str().unwrap();
    let input = tc["input"].as_str().unwrap();
    match parse_with_policy(input, &policy) {
        Ok(got) => match policy.resolve_action(&got) {
            Ok(plan) => {
                let mut result = parse_result(name, Ok(got));
                result["action"] =
                    json!({"action": plan.action, "command": plan.command, "args": plan.args});
                result
            }
            Err(_) => json!({"name": name, "ok": false}),
        },
        Err(_) => json!({"name": name, "ok": false}),
    }
}

fn parse_result(name: &str, result: Result<Uri, UriError>) -> Value {
    match result {
        Ok(got) => json!({
            "name": name,
            "ok": true,
            "uri": {
                "scheme": got.scheme,
                "namespace": got.namespace,
                "id": got.id,
                "query": got.query,
                "fragment": got.fragment,
                "original": got.original,
                "action": got.action,
                "canonical": got.canonical(),
                "vanity": got.vanity(),
            }
        }),
        Err(_) => json!({"name": name, "ok": false}),
    }
}

fn handler_case(tc: &Value) -> Value {
    let spec = handler_spec(&tc["spec"]);
    let name = tc["name"].as_str().unwrap();
    let platform = tc["platform"].as_str().unwrap();
    match (spec.handler_id(), snippet(platform, &spec)) {
        (Ok(handler_id), Ok(rendered)) => json!({
            "name": name,
            "ok": true,
            "handlerId": handler_id,
            "desktopFilename": if platform == "linux" { desktop_filename(&spec).unwrap() } else { String::new() },
            "rendered": rendered,
        }),
        _ => json!({"name": name, "ok": false}),
    }
}

fn invalid_handler_case(tc: &Value) -> Value {
    let spec = handler_spec(&tc["spec"]);
    json!({
        "name": tc["name"].as_str().unwrap(),
        "ok": spec.validate().is_err(),
        "handlerId": "",
        "desktopFilename": "",
        "rendered": "",
    })
}

fn handler_spec(value: &Value) -> HandlerSpec {
    HandlerSpec {
        vendor: value["vendor"].as_str().unwrap_or_default().to_string(),
        app: value["app"].as_str().unwrap_or_default().to_string(),
        instance: value["instance"].as_str().unwrap_or_default().to_string(),
        language: Language::from(value["language"].as_str().unwrap_or_default()),
        scheme: value["scheme"].as_str().unwrap_or_default().to_string(),
        version: value["version"].as_str().unwrap_or_default().to_string(),
        channel: value["channel"].as_str().unwrap_or_default().to_string(),
        app_path: value["appPath"].as_str().unwrap_or_default().to_string(),
        display_name: value["displayName"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    }
}
