#!/usr/bin/env python3
"""Offline validation for the immutable releases.json registry index."""
import json
import re
import sys
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


def main():
    src = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("releases.json")
    try:
        validate(json.loads(src.read_text(encoding="utf-8")))
    except (OSError, ValueError) as exc:
        print(f"release index invalid: {exc}", file=sys.stderr)
        return 1
    print(f"release index valid: {src}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
