#!/usr/bin/env python3
"""Validate release catalog syntax, additive changes, and optional Git source contents."""
import argparse
import json
import re
import subprocess
import sys
import tomllib
from pathlib import Path, PurePosixPath

COMMIT = re.compile(r"[0-9a-f]{40}\Z")
SEMVER = re.compile(r"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?\Z")
LEGACY = re.compile(r"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)\Z")
NAME = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]*\Z")


def validate(doc):
    if not isinstance(doc, dict) or doc.get("schema") != 1:
        raise ValueError("expected releases.json schema 1")
    packages = doc.get("packages")
    if not isinstance(packages, dict):
        raise ValueError("packages must be an object")
    for name, versions in packages.items():
        if not isinstance(name, str) or not NAME.fullmatch(name):
            raise ValueError(f"invalid package name: {name!r}")
        if not isinstance(versions, dict) or not versions:
            raise ValueError(f"{name}: at least one version is required")
        for version, entry in versions.items():
            if not isinstance(version, str) or not (SEMVER.fullmatch(version) or LEGACY.fullmatch(version)):
                raise ValueError(f"{name}: invalid release version {version!r}")
            if not isinstance(entry, dict) or set(entry) != {"commit", "path"}:
                raise ValueError(f"{name}@{version}: expected commit and path")
            commit, path = entry["commit"], entry["path"]
            if not isinstance(commit, str) or not COMMIT.fullmatch(commit):
                raise ValueError(f"{name}@{version}: commit must be a lowercase 40-character Git SHA")
            if not isinstance(path, str) or not path or "\\" in path:
                raise ValueError(f"{name}@{version}: invalid path")
            p = PurePosixPath(path)
            if p.is_absolute() or any(x in (".", "..", "") for x in path.split("/")):
                raise ValueError(f"{name}@{version}: unsafe package path")


def check_additive(current, baseline):
    for name, versions in baseline["packages"].items():
        for version, old in versions.items():
            fresh = current["packages"].get(name, {}).get(version)
            if fresh != old:
                raise ValueError(f"{name}@{version}: published release was removed or changed")


def git_read(ref, path):
    result = subprocess.run(["git", "show", f"{ref}:{path}"],
                            capture_output=True, check=False)
    return result.stdout if result.returncode == 0 else None


def check_sources(doc):
    for name, versions in doc["packages"].items():
        for version, record in versions.items():
            ref, base = record["commit"], record["path"]
            v2 = git_read(ref, f"{base}/pjpackage.toml")
            old = git_read(ref, f"{base}/package.toml")
            if v2 is not None:
                try:
                    manifest = tomllib.loads(v2.decode("utf-8"))
                except (UnicodeDecodeError, ValueError) as exc:
                    raise ValueError(f"{name}@{version}: invalid pinned v2 manifest: {exc}") from exc
                pkg = manifest.get("package", {})
                if pkg.get("schema") != 2 or pkg.get("name") != name or pkg.get("version") != version:
                    raise ValueError(f"{name}@{version}: pinned v2 schema/name/version mismatch")
            elif old is not None:
                try:
                    manifest = tomllib.loads(old.decode("utf-8"))
                except (UnicodeDecodeError, ValueError) as exc:
                    raise ValueError(f"{name}@{version}: invalid pinned legacy manifest: {exc}") from exc
                if str(manifest.get("version", "0.0.0")) != version:
                    raise ValueError(f"{name}@{version}: pinned legacy version mismatch")
            elif version != "0.0.0" or git_read(ref, f"{base}/init.pj") is None:
                raise ValueError(f"{name}@{version}: pinned commit/path lacks a matching package manifest")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("index", nargs="?", default="releases.json")
    parser.add_argument("--baseline", help="previous releases.json to enforce append-only records")
    parser.add_argument("--check-git", action="store_true", help="verify pinned files are in local Git history")
    args = parser.parse_args()
    try:
        current = json.loads(Path(args.index).read_text(encoding="utf-8"))
        validate(current)
        if args.baseline:
            baseline = json.loads(Path(args.baseline).read_text(encoding="utf-8"))
            validate(baseline)
            check_additive(current, baseline)
        if args.check_git:
            check_sources(current)
    except (OSError, ValueError) as exc:
        print(f"release index invalid: {exc}", file=sys.stderr)
        return 1
    print("release index valid; existing entries unchanged and pinned sources checked when requested")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
