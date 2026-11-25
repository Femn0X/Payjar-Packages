# pygame 0.1.0

An original Rust game-development core inspired by Pygame, not a port of the Python pygame project. MIT licensed. No Python, SDL, or external crate dependencies.

## Features

RGBA software surfaces, fill, clipped rectangles and circles, pixel access, clipped blitting (raw copy), rectangle collision, vectors, host-fed input event queue and fixed-step clock.

## Build and test

```sh
cargo build --release
cargo test
cargo check --target wasm32-unknown-unknown
```

Requires a Rust toolchain and the relevant target standard library. Intended for Windows, macOS, Linux and WebAssembly, including x86-64 and ARM64 where the Rust target supports std. Not verified on every target.

```rust
use pygame::{Surface, Rect};
let mut screen = Surface::new(640, 480).unwrap();
screen.fill([20, 20, 30, 255]);
screen.rect(Rect { x: 10, y: 20, w: 80, h: 40 }, [255, 100, 0, 255]);
```

## Integration status

This is a v2 Rust library package, not a directly importable Payjar module yet. A compatible pjpm Cargo adapter is required. The host must present RGBA pixels, translate native/browser input into events, and supply frame timing. Native windows, audio, image decoding, sprites and the Payjar runtime bridge are not implemented in this release. Headless rendering requires no display. No claim of full Pygame API compatibility.
