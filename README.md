# LiveSplit.AutoSplitters.WASM
Autosplitters for LiveSplit written for use with the new auto-splitting runtime

# Autospliters in this repository:
- Persona 4 Golden Load Remover (`p4g` folder)
- Saints Row 2 Autosplitter and Load Remover (`sr2` folder)
- GTA IV Autosplitter and Load Remover (`gta4` folder)
- Persona 3 Portable Load Remover (`p3p` folder)
- GTA 3 Autosplitter (`gta3` folder)

The latest compiled binaries are bundled into a single
[release](https://github.com/hoXyy/LiveSplit.AutoSplitters.WASM/releases/tag/latest).

# Manual compiling

Install the WebAssembly target:

```sh
rustup target add wasm32-unknown-unknown
```

Build every autosplitter from the repository root:

```sh
cargo build --workspace --release
```

To build only one autosplitter, select its package:

```sh
cargo build --release --package gta3-autosplitter
```

The compiled `.wasm` files are written to
`target/wasm32-unknown-unknown/release`.
