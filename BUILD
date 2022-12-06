load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@crates//:defs.bzl", "aliases", "all_crate_deps")

package(default_visibility = ["//visibility:public"])

config_setting(
    name = "is_rust_wasm",
    values = {
        "platforms": "@rules_rust//rust/platform:wasm",
    },
)

rust_library(
    name = "simple_ssr",
    srcs = [
        "src/lib.rs",
    ],
    deps = [
        "@crates//:yew",
        "@crates//:serde",
        "@crates//:uuid",
    ] + select({
        ":is_rust_wasm": [],
        "//conditions:default": [
            "@crates//:reqwest",
        ],
    }),
)

rust_binary(
    name = "simple_ssr_server",
    srcs = [
        "src/bin/simple_ssr_server.rs",
    ],
    deps = [
        ":simple_ssr",

        "@crates//:tokio",
        "@crates//:warp",
        "@crates//:clap",
        "@crates//:futures",
        "@crates//:bytes",
        "@crates//:yew",
    ],
    crate_features = [
		"simple_ssr/ssr",
        "yew/ssr",
    ],
)

rust_binary(
    name = "simple_ssr_hydrate",
    srcs = [
        "src/bin/simple_ssr_hydrate.rs",
    ],
    deps = [
        ":simple_ssr",

        "@crates//:yew",
        "@crates//:wasm-bindgen-futures",
        "@crates//:wasm-logger",
        "@crates//:log",
    ],
    crate_features = [
		"simple_ssr/hydration",
        "yew/hydration",
    ],
)

rust_wasm_bindgen(
    name = "simple_ssr_wasm",
    target = "web",
    wasm_file = ":simple_ssr_hydrate",
)
