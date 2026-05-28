#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
FIXTURE_DIR = ROOT / "spec" / "fixtures"
EXPECTED_FIXTURES = ["cite-contract.json", "handler-contract.json"]


def main() -> int:
    fixture_files = sorted(path.name for path in FIXTURE_DIR.glob("*.json"))
    if fixture_files != EXPECTED_FIXTURES:
        print(
            "parity: spec/fixtures must contain exactly "
            + ", ".join(EXPECTED_FIXTURES)
            + f"; got {fixture_files}",
            file=sys.stderr,
        )
        return 1

    outputs = {
        "go": run_json(
            "go",
            ["env", "-u", "GOROOT", "go", "run", "./tools/parity", "--fixtures", "../spec/fixtures"],
            cwd=ROOT / "go",
            env={"GOCACHE": os.environ.get("GOCACHE", "/tmp/cite-go-parity-build")},
        ),
        "ts": run_json("ts", ["node", "tools/parity/emitters/ts.js", "spec/fixtures"]),
        "py": run_json(
            "py",
            [sys.executable, "tools/parity/emitters/py.py", "spec/fixtures"],
            env={"PYTHONPATH": str(ROOT / "py" / "src")},
        ),
        "rs": run_json("rs", ["cargo", "run", "--quiet", "--bin", "parity", "--", "../spec/fixtures"], cwd=ROOT / "rs"),
        "php": run_json("php", ["php", "tools/parity/emitters/php.php", "spec/fixtures"]),
    }

    reference = outputs["go"]
    failed = False
    for language, output in outputs.items():
        if language == "go":
            continue
        if output != reference:
            failed = True
            print(f"parity: {language} differs from go", file=sys.stderr)
            print_diff(reference, output, [language])

    if failed:
        return 1

    cases = count_cases(reference)
    print(f"parity: ok go ts py rs php ({cases} cases)")
    return 0


def run_json(name: str, command: list[str], cwd: Path | None = None, env: dict[str, str] | None = None) -> Any:
    merged_env = os.environ.copy()
    if env:
        merged_env.update(env)
    proc = subprocess.run(
        command,
        cwd=str(cwd or ROOT),
        env=merged_env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        print(f"parity: {name} emitter failed: {' '.join(command)}", file=sys.stderr)
        if proc.stdout:
            print(proc.stdout, file=sys.stderr)
        if proc.stderr:
            print(proc.stderr, file=sys.stderr)
        sys.exit(proc.returncode)
    try:
        return json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        print(f"parity: {name} emitted invalid JSON: {exc}", file=sys.stderr)
        print(proc.stdout, file=sys.stderr)
        if proc.stderr:
            print(proc.stderr, file=sys.stderr)
        sys.exit(1)


def print_diff(left: Any, right: Any, path: list[str]) -> None:
    if type(left) is not type(right):
        print(f"  {'.'.join(path)} type {type(left).__name__} != {type(right).__name__}", file=sys.stderr)
        return
    if isinstance(left, dict):
        keys = sorted(set(left) | set(right))
        for key in keys:
            if key not in left:
                print(f"  {'.'.join(path + [str(key)])} missing from go", file=sys.stderr)
                return
            if key not in right:
                print(f"  {'.'.join(path + [str(key)])} missing from candidate", file=sys.stderr)
                return
            if left[key] != right[key]:
                print_diff(left[key], right[key], path + [str(key)])
                return
        return
    if isinstance(left, list):
        if len(left) != len(right):
            print(f"  {'.'.join(path)} len {len(left)} != {len(right)}", file=sys.stderr)
            return
        for index, (left_item, right_item) in enumerate(zip(left, right)):
            if left_item != right_item:
                print_diff(left_item, right_item, path + [str(index)])
                return
        return
    print(f"  {'.'.join(path)} {left!r} != {right!r}", file=sys.stderr)


def count_cases(output: dict[str, Any]) -> int:
    uri = output["uri"]
    handler = output["handler"]
    return sum(len(uri[key]) for key in ["valid", "invalid", "vanity", "action"]) + sum(
        len(handler[key]) for key in ["valid", "invalid"]
    )


if __name__ == "__main__":
    raise SystemExit(main())
