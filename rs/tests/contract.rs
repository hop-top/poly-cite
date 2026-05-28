use hop_top_cite::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct UriFixture {
    #[serde(rename = "namespacePolicy")]
    namespace_policy: NamespacePolicyFixture,
    #[serde(rename = "validCases")]
    valid_cases: Vec<UriValidCase>,
    #[serde(rename = "invalidCases")]
    invalid_cases: Vec<UriInvalidCase>,
    #[serde(rename = "vanityCases")]
    vanity_cases: Vec<VanityCase>,
    #[serde(rename = "actionCases")]
    action_cases: Vec<ActionCase>,
}

#[derive(Debug, Deserialize)]
struct NamespacePolicyFixture {
    #[serde(rename = "defaultNamespaceSegments")]
    default_namespace_segments: usize,
    #[serde(rename = "schemeNamespaceSegments")]
    scheme_namespace_segments: HashMap<String, usize>,
}

#[derive(Debug, Deserialize)]
struct UriValidCase {
    name: String,
    input: String,
    expected: ExpectedUri,
    canonical: String,
}

#[derive(Debug, Deserialize)]
struct UriInvalidCase {
    name: String,
    input: String,
}

#[derive(Debug, Deserialize)]
struct VanityCase {
    name: String,
    input: String,
    #[serde(default)]
    aliases: Vec<VanityAlias>,
    #[serde(default)]
    options: FixtureParseOptions,
    valid: bool,
    #[serde(default)]
    expected: ExpectedUri,
    #[serde(default)]
    canonical: String,
}

#[derive(Debug, Deserialize, Default)]
struct FixtureParseOptions {
    #[serde(default)]
    strict: bool,
    #[serde(default, rename = "jsonAmbiguity")]
    json_ambiguity: bool,
}

#[derive(Debug, Deserialize)]
struct ActionCase {
    name: String,
    input: String,
    #[serde(rename = "actionRoutes")]
    action_routes: HashMap<String, ActionRoute>,
    expected: ExpectedUri,
    canonical: String,
    command: String,
    args: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedUri {
    #[serde(default)]
    scheme: String,
    #[serde(default)]
    namespace: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    query: String,
    #[serde(default)]
    fragment: String,
    #[serde(default)]
    original: String,
    #[serde(default)]
    action: String,
}

#[derive(Debug, Deserialize)]
struct HandlerFixture {
    cases: Vec<HandlerCase>,
    #[serde(rename = "invalidCases")]
    invalid_cases: Vec<HandlerInvalidCase>,
}

#[derive(Debug, Deserialize)]
struct HandlerCase {
    name: String,
    platform: String,
    spec: HandlerSpecFixture,
    expected: HandlerExpected,
}

#[derive(Debug, Deserialize)]
struct HandlerInvalidCase {
    name: String,
    spec: HandlerSpecFixture,
}

#[derive(Debug, Deserialize)]
struct HandlerSpecFixture {
    #[serde(default)]
    vendor: String,
    #[serde(default)]
    app: String,
    #[serde(default)]
    instance: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    scheme: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    channel: String,
    #[serde(default, rename = "appPath")]
    app_path: String,
    #[serde(default, rename = "displayName")]
    display_name: String,
}

#[derive(Debug, Deserialize)]
struct HandlerExpected {
    #[serde(default, rename = "handlerId")]
    handler_id: String,
    #[serde(default, rename = "desktopFilename")]
    desktop_filename: String,
    #[serde(rename = "renderedContains")]
    rendered_contains: Vec<String>,
}

fn uri_fixture() -> UriFixture {
    serde_json::from_str(include_str!("../../spec/fixtures/cite-contract.json")).unwrap()
}

fn handler_fixture() -> HandlerFixture {
    serde_json::from_str(include_str!("../../spec/fixtures/handler-contract.json")).unwrap()
}

fn policy_from_fixture(fixture: &UriFixture) -> Policy {
    Policy {
        default_namespace_segments: fixture.namespace_policy.default_namespace_segments,
        scheme_namespace_segments: fixture.namespace_policy.scheme_namespace_segments.clone(),
        vanity_aliases: Vec::new(),
        action_routes: HashMap::new(),
    }
}

fn assert_uri(got: &Uri, expected: &ExpectedUri) {
    assert_eq!(got.scheme, expected.scheme);
    assert_eq!(got.namespace, expected.namespace);
    assert_eq!(got.id, expected.id);
    assert_eq!(got.query, expected.query);
    assert_eq!(got.fragment, expected.fragment);
    assert_eq!(got.original, expected.original);
    assert_eq!(got.action, expected.action);
}

fn handler_spec(spec: &HandlerSpecFixture) -> HandlerSpec {
    HandlerSpec {
        vendor: spec.vendor.clone(),
        app: spec.app.clone(),
        instance: spec.instance.clone(),
        language: Language::from(spec.language.as_str()),
        scheme: spec.scheme.clone(),
        version: spec.version.clone(),
        channel: spec.channel.clone(),
        app_path: spec.app_path.clone(),
        display_name: spec.display_name.clone(),
    }
}

#[test]
fn uri_contract_valid_cases() {
    let fixture = uri_fixture();
    let policy = policy_from_fixture(&fixture);

    for case in fixture.valid_cases {
        let got = parse_with_policy(&case.input, &policy).unwrap_or_else(|err| {
            panic!("{} failed: {err}", case.name);
        });
        assert_uri(&got, &case.expected);
        assert_eq!(got.canonical(), case.canonical);
        assert_eq!(got.to_string(), case.canonical);
    }
}

#[test]
fn uri_contract_invalid_cases() {
    let fixture = uri_fixture();
    let policy = policy_from_fixture(&fixture);

    for case in fixture.invalid_cases {
        assert!(
            parse_with_policy(&case.input, &policy).is_err(),
            "{} should fail",
            case.name
        );
    }
}

#[test]
fn uri_contract_vanity_cases() {
    let fixture = uri_fixture();

    for case in &fixture.vanity_cases {
        let mut policy = policy_from_fixture(&fixture);
        policy.vanity_aliases = case.aliases.clone();
        let options = ParseOptions {
            strict: case.options.strict,
            json_ambiguity: case.options.json_ambiguity,
        };
        let got = parse_with_policy_options(&case.input, &policy, options);
        if !case.valid {
            assert!(got.is_err(), "{} should fail", case.name);
            continue;
        }
        let got = got.unwrap_or_else(|err| panic!("{} failed: {err}", case.name));
        assert_uri(&got, &case.expected);
        assert_eq!(got.canonical(), case.canonical);
        assert_eq!(got.vanity(), case.expected.original);
    }
}

#[test]
fn uri_contract_action_cases() {
    let fixture = uri_fixture();

    for case in &fixture.action_cases {
        let mut policy = policy_from_fixture(&fixture);
        policy.action_routes = case.action_routes.clone();
        let got = parse_with_policy(&case.input, &policy).unwrap_or_else(|err| {
            panic!("{} failed: {err}", case.name);
        });
        assert_eq!(got.scheme, case.expected.scheme);
        assert_eq!(got.namespace, case.expected.namespace);
        assert_eq!(got.id, case.expected.id);
        assert_eq!(got.action, case.expected.action);
        assert_eq!(got.canonical(), case.canonical);

        let resolved = policy.resolve_action(&got).unwrap();
        assert_eq!(resolved.action, got.action);
        assert_eq!(resolved.command, case.command);
        assert_eq!(resolved.args, case.args);
    }
}

#[test]
fn action_conflicts_fail() {
    let policy = Policy::default();
    let err = parse_with_policy(
        "tlc://org/repo/T-0001?action=task.claim&cmd=task&verb=drop",
        &policy,
    )
    .unwrap_err();
    assert!(matches!(err, UriError::ConflictingActionQueryParameters));
}

#[test]
fn ambiguous_fuzzy_vanity_can_render_json() {
    let policy = Policy {
        vanity_aliases: vec![
            VanityAlias {
                from: "task://shortcuta".to_string(),
                to: "task://hop-top/cite/T-0001".to_string(),
                prefix: false,
                preserve_suffix: false,
            },
            VanityAlias {
                from: "task://shortcutb".to_string(),
                to: "task://hop-top/cite/T-0002".to_string(),
                prefix: false,
                preserve_suffix: false,
            },
        ],
        ..Default::default()
    };
    let err = parse_with_policy_options(
        "task://shortcut",
        &policy,
        ParseOptions {
            strict: false,
            json_ambiguity: true,
        },
    )
    .unwrap_err();
    match &err {
        UriError::AmbiguousVanity(ambiguous) => assert_eq!(ambiguous.candidates.len(), 2),
        other => panic!("unexpected error: {other}"),
    }
    let value: serde_json::Value = serde_json::from_str(&err.to_string()).unwrap();
    assert_eq!(value["input"], "task://shortcut");
    assert_eq!(value["candidates"].as_array().unwrap().len(), 2);
}

#[test]
fn registry_parse_complete_and_types() {
    let mut registry = Registry::new();
    assert!(registry.parse("task://hop-top/cite/T-0001").is_err());

    registry
        .register(TypeRegistration {
            name: "task".to_string(),
            parser: None,
            completer: Some(Box::new(|prefix| {
                Ok(["T-0001", "T-0002", "T-0099"]
                    .into_iter()
                    .filter(|item| item.starts_with(prefix))
                    .map(str::to_string)
                    .collect())
            })),
        })
        .unwrap();
    registry
        .register(TypeRegistration {
            name: "repo".to_string(),
            parser: Some(Box::new(|_| {
                Ok(Uri {
                    scheme: "repo".to_string(),
                    namespace: "custom".to_string(),
                    id: "parsed".to_string(),
                    ..Uri::default()
                })
            })),
            completer: None,
        })
        .unwrap();

    let got = registry.parse("task://hop-top/cite/T-0001").unwrap();
    assert_eq!(got.namespace, "hop-top/cite");
    assert_eq!(got.id, "T-0001");

    let got = registry.parse("repo://hop-top/cite").unwrap();
    assert_eq!(got.namespace, "custom");
    assert_eq!(got.id, "parsed");

    assert_eq!(
        registry.complete("task", "T-000").unwrap().unwrap(),
        vec!["T-0001", "T-0002"]
    );
    assert_eq!(registry.complete("repo", "").unwrap(), None);
    assert_eq!(
        registry.types(),
        vec!["repo".to_string(), "task".to_string()]
    );

    let out = complete_with_scheme(&registry, "task", "task://T-").unwrap();
    assert_eq!(
        out.suggestions,
        vec!["task://T-0001", "task://T-0002", "task://T-0099"]
    );
}

#[test]
fn handler_contract_cases() {
    let fixture = handler_fixture();

    for case in fixture.cases {
        let spec = handler_spec(&case.spec);
        assert_eq!(spec.handler_id().unwrap(), case.expected.handler_id);
        if !case.expected.desktop_filename.is_empty() {
            assert_eq!(
                desktop_filename(&spec).unwrap(),
                case.expected.desktop_filename
            );
        }
        let rendered = snippet(&case.platform, &spec).unwrap_or_else(|err| {
            panic!("{} failed: {err}", case.name);
        });
        for expected in case.expected.rendered_contains {
            assert!(
                rendered.contains(&expected),
                "{} missing {expected:?} in {rendered:?}",
                case.name
            );
        }
    }
}

#[test]
fn handler_contract_invalid_cases() {
    let fixture = handler_fixture();

    for case in fixture.invalid_cases {
        let spec = handler_spec(&case.spec);
        assert!(spec.validate().is_err(), "{} should fail", case.name);
    }
}

#[test]
fn handler_unknown_platform_and_patch_plist() {
    let spec = HandlerSpec {
        vendor: "hop-top".to_string(),
        app: "scheme".to_string(),
        instance: String::new(),
        language: Language::Go,
        scheme: "task".to_string(),
        version: "0.2.0-alpha.0".to_string(),
        channel: "alpha".to_string(),
        app_path: "/usr/bin/task".to_string(),
        display_name: "Hop Task Handler".to_string(),
    };
    assert!(snippet("amiga", &spec).is_err());

    let patched = patch_plist(
        "<?xml version=\"1.0\"?>\n<plist version=\"1.0\">\n<dict>\n</dict>\n</plist>".as_bytes(),
        &spec,
    )
    .unwrap();
    assert!(patched.contains("CFBundleURLSchemes"));
    assert!(patched.contains("hop-top.scheme.go.task"));
}
