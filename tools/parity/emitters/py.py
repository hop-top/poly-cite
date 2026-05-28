from __future__ import annotations

import json
import sys
from pathlib import Path

from cite import ActionRoute, HandlerSpec, ParseOptions, Policy, VanityAlias, desktop_filename, parse, resolve_action, snippet

fixture_dir = Path(sys.argv[1] if len(sys.argv) > 1 else "spec/fixtures")
cite_fixture = json.loads((fixture_dir / "cite-contract.json").read_text())
handler_fixture = json.loads((fixture_dir / "handler-contract.json").read_text())
base_policy = Policy(
    default_namespace_segments=cite_fixture["namespacePolicy"]["defaultNamespaceSegments"],
    scheme_namespace_segments=cite_fixture["namespacePolicy"]["schemeNamespaceSegments"],
)


def cite_out(got):
    return {
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


def parse_result(name: str, fn):
    try:
        got = fn()
    except Exception:
        return {"name": name, "ok": False}
    return {"name": name, "ok": True, "uri": cite_out(got)}


def invalid_parse_result(name: str, fn):
    try:
        fn()
    except Exception:
        return {"name": name, "ok": True}
    return {"name": name, "ok": False}


def handler_spec(value: dict) -> HandlerSpec:
    return HandlerSpec.from_mapping(value)


def handler_result(tc: dict):
    try:
        spec = handler_spec(tc["spec"])
        return {
            "name": tc["name"],
            "ok": True,
            "handlerId": spec.handler_id(),
            "desktopFilename": desktop_filename(spec) if tc["platform"] == "linux" else "",
            "rendered": snippet(tc["platform"], spec),
        }
    except Exception:
        return {"name": tc["name"], "ok": False}


def invalid_handler_result(tc: dict):
    try:
        handler_spec(tc["spec"]).validate()
    except Exception:
        return {"name": tc["name"], "ok": True, "handlerId": "", "desktopFilename": "", "rendered": ""}
    return {"name": tc["name"], "ok": False, "handlerId": "", "desktopFilename": "", "rendered": ""}


output = {
    "version": 1,
    "uri": {
        "valid": [parse_result(tc["name"], lambda tc=tc: parse(tc["input"], base_policy)) for tc in cite_fixture["validCases"]],
        "invalid": [invalid_parse_result(tc["name"], lambda tc=tc: parse(tc["input"], base_policy)) for tc in cite_fixture["invalidCases"]],
        "vanity": [
            parse_result(
                tc["name"],
                lambda tc=tc: parse(
                    tc["input"],
                    Policy(
                        default_namespace_segments=base_policy.default_namespace_segments,
                        scheme_namespace_segments=base_policy.scheme_namespace_segments,
                        vanity_aliases=[VanityAlias.from_mapping(alias) for alias in tc.get("aliases", [])],
                    ),
                    ParseOptions.from_mapping(tc.get("options")),
                ),
            )
            for tc in cite_fixture["vanityCases"]
        ],
        "action": [],
    },
    "handler": {
        "valid": [handler_result(tc) for tc in handler_fixture["cases"]],
        "invalid": [invalid_handler_result(tc) for tc in handler_fixture["invalidCases"]],
    },
}

for tc in cite_fixture["actionCases"]:
    routes = {name: ActionRoute.from_mapping(route) for name, route in tc["actionRoutes"].items()}
    policy = Policy(
        default_namespace_segments=base_policy.default_namespace_segments,
        scheme_namespace_segments=base_policy.scheme_namespace_segments,
        action_routes=routes,
    )
    result = parse_result(tc["name"], lambda tc=tc, policy=policy: parse(tc["input"], policy))
    if result["ok"]:
        got = parse(tc["input"], policy)
        plan = resolve_action(got, policy)
        result["action"] = {"action": plan.action, "command": plan.command, "args": plan.args}
    output["uri"]["action"].append(result)

print(json.dumps(output, separators=(",", ":")))
