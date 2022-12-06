This is the SSR example from the Yew repository, I'm trying to get to work via bazel.

## Non-bazel

```shell
trunk build index.html
```

First, I inspected what commands `trunk build` actually runs, by cloning and editing that repository. It runs
```shell
cargo build \
        --target=wasm32-unknown-unknown \
        --manifest-path /Users/mcrane/tmp/yew-bazel/Cargo.toml \
        --bin simple_ssr_hydrate \
        --features hydration
```
to build the binary, and then invokves wasm-bindgen
```shell
/Users/mcrane/Library/Caches/dev.trunkrs.trunk/wasm-bindgen-0.2.83/wasm-bindgen \
        --target=web \
        --out-dir=/Users/mcrane/tmp/yew-bazel/target/wasm-bindgen/debug \
        --out-name=simple_ssr_hydrate-1b6f311a7f3e394a \
        /Users/mcrane/tmp/yew-bazel/target/wasm32-unknown-unknown/debug/simple_ssr_hydrate.wasm \
        --no-typescript
```

This works, and doesn't complain about missing functions, packages, etc.
Full output (incl debug of self) here - https://gist.github.com/snapbug/8d367a69c9151ff86f853f0188c69f89

## Bazel v1

*Using explicit package management*

In this repository, branch = `main`.

Ran:
```shell
CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index
bazel build //:simple_ssr_wasm
```

Error:
```
error[E0425]: cannot find function `fetch_uuid` in this scope
```

Full command (using `-s` option to `bazel build`): 
https://gist.github.com/snapbug/8ea1184e7dd670dd4aee86d2b19894d4

## Bazel v2

*Using all_crate_deps*

In this repositry, branch = `all_crate_deps`

Ran:
```shell
CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index
bazel build //:simple_ssr_wasm
```

Error:
```
ERROR: Traceback (most recent call last):
        File "/Users/mcrane/tmp/yew-bazel/BUILD", line 19, column 26, in <toplevel>
                deps = all_crate_deps(normal=True),
        File "/private/var/tmp/_bazel_mcrane/4e614d5950bc0246fd5af27b4ad5cf7a/external/crates/defs.bzl", line 206, column 62, in all_crate_deps
                crate_deps += selects.with_or({_CONDITIONS[condition]: deps.values()})
Error: unhashable type: 'list'
```


## Bazel v2.1

Attempting to fix the error above by changing the offending line to explicitly add `tuple` instead of list
```python
                crate_deps += selects.with_or({tuple(_CONDITIONS[condition]): deps.values()})
```

Error
```
Analyzing: target //:simple_ssr_wasm (1 packages loaded, 0 targets configured)
ERROR: /Users/mcrane/tmp/yew-bazel/BUILD:36:12: //:aarch64-apple-darwin is not a valid select() condition for //:simple_ssr_hydrate.
To inspect the select(), run: bazel query --output=build //:simple_ssr_hydrate.
```

The output mentioned:
```
❯ bazel query --output=build //:simple_ssr_wasm 
# /Users/mcrane/tmp/yew-bazel/BUILD:50:18
rust_wasm_bindgen(
  name = "simple_ssr_wasm",
  target = "web",
  wasm_file = "//:simple_ssr_hydrate",
)
# Rule simple_ssr_wasm instantiated at (most recent call last):
#   /Users/mcrane/tmp/yew-bazel/BUILD:50:18 in <toplevel>
# Rule rust_wasm_bindgen defined at (most recent call last):
#   /private/var/tmp/_bazel_mcrane/4e614d5950bc0246fd5af27b4ad5cf7a/external/rules_rust/wasm_bindgen/wasm_bindgen.bzl:148:25 in <toplevel>

Loading: 0 packages loaded
```
