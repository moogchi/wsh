#!/usr/bin/env python3
"""Build wsh, compile the C e2e test harnesses, and run them.

Each tests/e2e/test_*.c file is a standalone C program: it forks/execs the
wsh binary over pipes, drives it like a user would, and asserts on its
output. This script builds wsh, compiles every test_*.c against
tests/e2e/common.c with gcc, then runs each resulting binary with WSH_BIN
pointed at the freshly built shell.
"""

import argparse
import os
import pathlib
import subprocess
import sys

REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent
E2E_DIR = REPO_ROOT / "tests" / "e2e"
BUILD_DIR = E2E_DIR / "build"
COMMON_SRC = E2E_DIR / "common.c"

TEST_TIMEOUT_SECONDS = 20


def run(cmd, **kwargs):
    print(f"$ {' '.join(str(c) for c in cmd)}")
    return subprocess.run(cmd, cwd=REPO_ROOT, **kwargs)


def build_wsh(release: bool) -> pathlib.Path:
    cmd = ["cargo", "build"]
    if release:
        cmd.append("--release")

    result = run(cmd)
    if result.returncode != 0:
        print("FAIL  build: cargo build failed", file=sys.stderr)
        sys.exit(1)

    profile = "release" if release else "debug"
    wsh_bin = REPO_ROOT / "target" / profile / "wsh"
    if not wsh_bin.exists():
        print(f"FAIL  build: expected binary not found at {wsh_bin}", file=sys.stderr)
        sys.exit(1)

    print(f"PASS  build: {wsh_bin}")
    return wsh_bin


def discover_tests():
    return sorted(E2E_DIR.glob("test_*.c"))


def compile_test(src: pathlib.Path):
    BUILD_DIR.mkdir(parents=True, exist_ok=True)
    out = BUILD_DIR / src.stem
    cmd = [
        "gcc",
        "-std=c11",
        "-Wall",
        "-Wextra",
        "-O0",
        "-g",
        str(src),
        str(COMMON_SRC),
        "-o",
        str(out),
    ]
    result = run(cmd)
    if result.returncode != 0:
        print(f"FAIL  compile: {src.name}", file=sys.stderr)
        return None
    return out


def run_test(binary: pathlib.Path, wsh_bin: pathlib.Path) -> bool:
    env = dict(os.environ)
    env["WSH_BIN"] = str(wsh_bin)
    try:
        result = subprocess.run([str(binary)], env=env, timeout=TEST_TIMEOUT_SECONDS)
    except subprocess.TimeoutExpired:
        print(f"FAIL  {binary.name}: timed out after {TEST_TIMEOUT_SECONDS}s", file=sys.stderr)
        return False
    return result.returncode == 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--release", action="store_true", help="build wsh with `cargo build --release`"
    )
    args = parser.parse_args()

    wsh_bin = build_wsh(args.release)

    tests = discover_tests()
    if not tests:
        print("no e2e tests found under tests/e2e/test_*.c", file=sys.stderr)
        sys.exit(1)

    failures = []
    for src in tests:
        binary = compile_test(src)
        if binary is None:
            failures.append(src.name)
            continue

        ok = run_test(binary, wsh_bin)
        status = "PASS" if ok else "FAIL"
        print(f"{status}  {src.name}")
        if not ok:
            failures.append(src.name)

    print()
    passed = len(tests) - len(failures)
    print(f"{passed}/{len(tests)} e2e tests passed")

    if failures:
        print("failed: " + ", ".join(failures))
        sys.exit(1)


if __name__ == "__main__":
    main()
