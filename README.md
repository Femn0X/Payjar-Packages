# Payjar-Packages

Community package registry for [Payjar](https://github.com/Femn0X/Payjar).

This registry is transitioning to the universal package format described in [FORMAT.md](FORMAT.md). The transition is additive: existing packages remain valid and old clients can continue using the legacy layout.

## Package formats

### Universal v2 package (new format)

A v2 package contains a `pjpackage.toml` manifest with `[package]` and `schema = 2`. It may be written in Python, JavaScript, Rust, Go, Java, C/C++, Payjar, or another language for which a supported adapter exists.

```text
my-package/
├── pjpackage.toml
├── src/
└── README.md
```

The manifest describes the language, package kind, build adapter, runtime entry, and Payjar registry dependencies. A v2 package does not require `init.pj`.

### Legacy Payjar package (still supported)

Existing packages may continue using:

```text
geometry/
├── init.pj
└── package.toml
```

The current legacy manifest uses top-level keys such as `description`, `language`, `build`, and `version`. Keep these files unchanged while old clients are supported. Legacy packages containing only `init.pj` also remain valid.

## Publishing

The legacy `pjpm upload` workflow remains supported by existing clients. The v2 publishing workflow will validate the manifest, package contents, version, and archive digest before publishing. Until v2 client support is released, do not assume a v2 package can be installed by the currently released `pjpm`.

## Registry contribution rules

- Use a lowercase package slug matching `package.name`.
- Use semantic versions for v2 packages.
- Include a README and license information.
- Do not commit generated build output, caches, virtual environments, or dependency directories.
- Declare required toolchains and platform constraints.
- Never include secrets, access tokens, or private keys.
- Build commands must be expressed as adapter plus argument arrays; arbitrary shell strings are not part of the v2 format.

## Compatibility policy

- `pjpackage.toml` with schema 2 is canonical for new packages.
- `package.toml` remains the legacy manifest.
- `init.pj` remains the legacy entry point and is not required by v2.
- Do not remove or rename legacy package files as part of the transition.

See [FORMAT.md](FORMAT.md) for the manifest specification and migration rules.

## Versioned releases

Newer clients can discover immutable historical versions through [`releases.json`](releases.json). Every release points to a full Git commit SHA and a package path at that commit; the old root package directories remain compatible with existing clients. See [RELEASES.md](RELEASES.md) for the catalog schema, publishing sequence, and security limitations.
