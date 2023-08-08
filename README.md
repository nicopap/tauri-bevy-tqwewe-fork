# Bevy Tauri experiment nº1

fork of <https://github.com/tqwewe/bevy-editor> updated for bevy 0.11,
experimenting with Bevy & Tauri integration.

## Architecture

Note that this is the original architecture, not mine.

This is split in 3:

1. A `viewport` crate, supposedly a bevy "component" that fits into the svelt app.
2. `src`, the JS-side of the standard tauri app architecture
3. `src-tauri`, the tauri "core", native code impl that can interact with the JS
   code.

### `viewport`

`viewport` is a full-on bevy app. It handles input and spawning, in short:
everything.

### JS code

Seems to be a boilerplate "hello world" Svelte App. Weirdly it spawns a dev server?
Not sure why. Seemingly it does not have any logic, basically just fluff.

## Limitations

* Since the linux webview implementation used by tauri is webkit2gtk, and
  webkit2's webgl2 implementation is not supported by bevy, and webkit2
  doesn't yet support WebGPU, this doesn't work on linux.
* I get a bunch of "out of bound memory access" errors. Seemingly in the
  input handler & also `bevy_ecs` code
  * There is probably something I'm doing wrong here, because I've tried
    a similar setup with my own crate (`bevy_mod_paramap`) and the wasm
    file went as far as showing something on screen and panicked only afterward
    (maybe because it was compiled with an older version of bevy?)
* This can't take advantage of bevy's multithreaded nature, since it is disabled
  in web builds.
* As far as I can see, there is **no communication** between bevy and the tauri
  web-app. So you can't use it as an editor

Basically this is a failed attempt.


## Setup

**Prerequisites**

- See https://tauri.studio/docs/getting-started/prerequisites
- WASM target `rustup target install wasm32-unknown-unknown`

Currently, you'll need to clone this repository and run it manually.

1. Install dependencies

```bash
$ yarn install
```

2. Build viewport

```bash
$ yarn build:viewport
```

3. Run app

```bash
$ yarn tauri dev
```
