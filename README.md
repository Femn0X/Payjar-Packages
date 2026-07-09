# Payjar-Packages

This is the community package registry for [Payjar](https://github.com/Femn0X/Payjar) —
a folder of installable packages that `pjpm`, Payjar's package manager, can
fetch, list, search, and publish to.

You don't clone this repo by hand. Use `pjpm` from your Payjar project instead:

```bash
./pjpm.sh list                 # see everything available here
./pjpm.sh search geo           # filter by name
./pjpm.sh install geometry     # copies geometry/ into your project
```

## What's a package?

A package is just a top-level folder in this repo containing an `init.pj` —
the same package format Payjar already uses for local packages (see
`geometry/` in the main Payjar repo for an example):

```
mathx/
├── init.pj            # required — package name + its func/class defs
└── pjpackage.toml      # optional — description + build metadata
```

```payjar
// mathx/init.pj
package mathx;

func double(n:Int)->Int{
 return n*2;
}
```

Once installed (`./pjpm.sh install mathx`), a project can use it with:

```payjar
import mathx;
public class main(@self){
 println(mathx.double(21));
}
```

## `pjpackage.toml` (optional)

```toml
description = "Extra math helpers"
language    = "payjar"   # or "rust", "c", "python", whatever the build needs
build       = "cargo build --release"   # optional, run after install
```

- `description` — one line, shown by `pjpm list`.
- `language` — informational only, shown alongside the build step.
- `build` — a shell command `pjpm install` runs inside the package's folder
  right after copying it in. **A package's native half can be written in any
  language** — pjpm doesn't inspect or execute anything itself beyond
  running this one command. It just needs `init.pj` (plus whatever the build
  produces) to be usable once the command finishes. Leave it out entirely
  for a pure-Payjar package with no build step.

## Publishing a package

From inside your own project, once you have a package folder ready locally:

```bash
./pjpm.sh upload mypackage "add mypackage"
```

This copies your local `./mypackage` into this registry, commits it, and
pushes — you'll need push access to this repo (an SSH key or git credential
configured for it). If nothing in the folder changed since the last upload,
`pjpm` says so and skips the commit.

## Guidelines for contributed packages

- Keep the top-level folder name short, lowercase, and matching the
  `package <name>;` declaration inside `init.pj`.
- Don't commit build output (`target/`, `__pycache__/`, `node_modules/`,
  compiled binaries, etc.) — only source and the `build` command needed to
  produce it.
- If your package wraps a native library, document any system dependencies
  the `build` command assumes (a compiler, an interpreter, a package
  manager) in your `description` or a short `NOTES.md` inside the package
  folder.
- One package, one folder, one purpose — split unrelated functionality into
  separate packages rather than bundling everything together.

## License

Unless a package's own folder says otherwise, contributions to this
registry are assumed to be shared under the same license as Payjar itself
(MIT). Check the license of any package before depending on it in something
you plan to redistribute.
