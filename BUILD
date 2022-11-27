load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")

package(default_visibility = ["//visibility:public"])

config_setting(
	name = "wasm_build",
	constraint_values = [
		"@platforms//cpu:wasm32",
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
		":wasm_build": [
		],
		"//conditions:default": [
			"@crates//:reqwest",
		],
	}),
	crate_features = [] + select({
		":wasm_build": [
			"hydration",
			"yew/hydration",
		],
		"//conditions:default": [
			"ssr",
			"yew/ssr",
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
		"ssr",
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
	],
)

rust_wasm_bindgen(
	name = "simple_ssr_wasm",
	target = "web",
	wasm_file = ":simple_ssr_hydrate",
)
