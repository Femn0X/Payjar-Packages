# Universal package format (v2)

Status: proposed format baseline. A manifest alone does not make an old `pjpm` binary understand v2; client support is being implemented in phases.

## Required manifest

File name: `pjpackage.toml`.

```toml
[package]
schema = 2
name = "sample"
version = "1.0.0"
description = "Example package"
kind = "library"
language = "python"

[build]
adapter = "python"
args = ["-m", "build"]

[runtime]
adapter = "python"
entry = "src/sample.py"

[dependencies]
# Other packages in this registry, expressed as SemVer constraints.
```

## Package fields

| Field | Required | Description |
| --- | --- | --- |
| `package.schema` | yes | Integer schema version; currently 2 |
| `package.name` | yes | Lowercase registry slug |
| `package.version` | yes | Semantic version |
| `package.kind` | yes | `library`, `executable`, `plugin`, `native`, `wasm`, or `source` |
| `package.language` | yes | Primary language/adapter identifier |
| `package.description` | no | Short package summary |
| `build.adapter` | when building | Registered adapter identifier |
| `build.args` | no | Argument vector passed to the adapter, never a shell string |
| `runtime.adapter` | when runnable | Registered runtime adapter |
| `runtime.entry` | when runnable | Package-relative entry path |
| `dependencies` | no | Payjar registry dependencies and SemVer constraints |

All paths must be relative to the package root. Absolute paths, parent traversal, and symlinks escaping the package root are invalid.

## Example: Python library

```toml
[package]
schema = 2
name = "py-math"
version = "1.0.0"
description = "Math helpers written in Python"
kind = "library"
language = "python"

[build]
adapter = "python"
args = ["-m", "build"]

[runtime]
adapter = "python"
entry = "src/py_math.py"
```

## Example: Rust executable

```toml
[package]
schema = 2
name = "rust-tool"
version = "1.0.0"
kind = "executable"
language = "rust"

[build]
adapter = "cargo"
args = ["build", "--release"]

[runtime]
adapter = "process"
entry = "target/release/rust-tool"
```

The adapter owns exact build/install behavior and validates the manifest before running tools. A package manifest must not supply a shell command string.

## Language support versus language interoperability

A package can be packaged and built in a language without being directly importable by the Payjar interpreter. Cross-language calls require a supported bridge, such as JSON request/response over a child process, WebAssembly/WASI with a declared interface, or a native library with an explicit platform and ABI. Do not assume direct imports of foreign-language source files.

## Legacy migration

Clients must detect manifests in this order:

1. `pjpackage.toml` with `package.schema = 2`.
2. Legacy `package.toml` with top-level fields.
3. `init.pj` without a manifest.

The legacy schema and its current semantics are frozen for compatibility. Existing packages do not need to migrate.
