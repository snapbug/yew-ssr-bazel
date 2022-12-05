load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "dd79bd4e2e2adabae738c5e93c36d351cf18071ff2acf6590190acf4138984f6",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.14.0/rules_rust-v0.14.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rust_repository_set", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()
rust_register_toolchains(
    edition="2021",
    extra_target_triples = [
        "wasm32-unknown-unknown",
    ],
)

load("@rules_rust//wasm_bindgen:repositories.bzl", "rust_wasm_bindgen_repositories", "rust_wasm_bindgen_register_toolchains")
rust_wasm_bindgen_repositories()
rust_wasm_bindgen_register_toolchains()

#
# Crates
#
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
crate_universe_dependencies()

load("//:crates.bzl", "cargo_dependencies")
cargo_dependencies()

load("@crates//:defs.bzl", "crate_repositories")
crate_repositories()
