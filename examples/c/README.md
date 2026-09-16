# Wickra Shazam — C / C++ examples

The Wickra Shazam C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_shazam.h`](../../bindings/c/include/wickra_shazam.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_shazam.hpp`](../../bindings/c/include/wickra_shazam.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-shazam-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_shazam.so`     | `-lwickra_shazam` |
| macOS    | `libwickra_shazam.dylib`  | `-lwickra_shazam` |
| Windows (MSVC) | `wickra_shazam.dll` | `wickra_shazam.dll.lib` (import lib) |

A static library (`libwickra_shazam.a` / `wickra_shazam.lib`) is emitted alongside.

## Build and run the examples

With CMake, as the CI C ABI job does:

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

## The examples

| Example | What it does |
|---------|--------------|
| `match.c` | A minimal C example: index a history and match the current state through the |
| `match.cpp` | A minimal C++ example: index a history, label a window, match the current state -- and show that the label lands the same whether it was sent before or after the index was built -- all through the C++ |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_shazam.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
