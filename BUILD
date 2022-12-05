load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")

package(default_visibility = ["//visibility:public"])

config_setting(
	name = "is_rust_wasm",
	constraint_values = [
		"@platforms//cpu:wasm32",
		"@rules_rust//rust/platform/os:unknown",
	],
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
			":is_rust_wasm": [
			],
			"//conditions:default": [
				"@crates//:reqwest",
			],
		},
		no_match_error = "not wasm",
	),
	crate_features = [] + select({
			":is_rust_wasm": [
				"hydration",
				"yew/hydration",
			],
			"//conditions:default": [
				"ssr",
				"yew/ssr",
			],
		},
		no_match_error = "not wasm?",
	),
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
		"ssr",
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
		"hydration",
		"yew/hydration",
	],
)

rust_wasm_bindgen(
	name = "simple_ssr_wasm",
	target = "web",
	wasm_file = ":simple_ssr_hydrate",
)
