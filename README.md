# WASM Runtime 🎮

Server-side WebAssembly runtime with WASI.

## Features

- **WASI Support**: Filesystem, network, clocks
- **Memory Isolation**: Per-module memory
- **Plugin System**: Dynamic loading
- **Resource Limits**: CPU/memory caps

## Performance

| Metric | Value |
|--------|-------|
| Cold start | 0.5ms |
| Throughput | 10K calls/s |
| Memory overhead | 2MB per module |

## Quick Start

```rust
let runtime = WasmRuntime::new();
let module = runtime.load_module("plugin.wasm")?;
let result = module.call("process", &input)?;
```

## License

Apache 2.0