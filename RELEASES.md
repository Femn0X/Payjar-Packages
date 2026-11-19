# Immutable versioned releases

The registry's `releases.json` is an additive release catalog (schema 1). Each version references a **full Git commit SHA** and a **package directory inside that commit**. This enables installations of old releases after `main` changes.

```json
{
  "schema": 1,
  "packages": {
    "Geometry": {
      "1.0": {
        "commit": "f014481e93602755d7bcbfa25693654b6e46f99c",
        "path": "Geometry"
      }
    }
  }
}
```

The `Geometry@1.0` entry is the existing, unmodified legacy package at a fixed commit. Its legacy `package.toml` uses version `1.0`, not SemVer; only legacy releases may use two-component versions. New v2 releases must use SemVer (`1.2.3`, etc.).

## Publishing a new version manually

1. Commit the complete package contents under a stable path (for example, `releases/hello/1.0.0/`), with a v2 `pjpackage.toml` whose version and name match the release.
2. Note the **40-character commit SHA** of that content commit.
3. In a later commit, append a new `packages.<name>.<version>` entry to `releases.json` containing that SHA and the directory path. Do not reuse a branch name or mutable tag as a release ref.
4. Run `python scripts/validate_releases.py releases.json --check-git`, open a PR, and verify the CI check. The check verifies that the pinned manifest exists and matches the recorded version.

**Existing release entries must never be changed or deleted.** Corrections require a new version. CI checks schema, safe paths, pinned manifest contents available in the checked-out Git history, and that existing version entries have not been rewritten or removed. It does **not** independently audit package code, prove all commits remain available forever, or cryptographically sign releases. Keep pinned commits reachable from the default branch; review is still required.

## Legacy compatibility

The original `<package>/package.toml` and `<package>/init.pj` root layout remains untouched for old clients. Updated `pjpm` clients prefer indexed immutable versions when present and can fall back to unindexed root packages when the catalog lacks an entry. Package source archives are obtained from the pinned commit, not the current `main` branch. The index is discovery metadata, not a signed registry.
