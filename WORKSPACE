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

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")
crates_repository(
    name = "crates",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.bzl.lock",
    manifests = ["//:Cargo.toml"],
    # packages = {
    #     "yew": crate.spec(version = "0.20.0", features = ["ssr", "hydration"]),
    #     "reqwest": crate.spec(version = "0.11.13", features = ["json"]),
    #     "serde": crate.spec(version = "1.0.148", features = ["derive"]),
    #     "uuid": crate.spec(version = "1.2.2", features = ["serde"]),
    #     "futures": crate.spec(version = "0.3.25"),
    #     "bytes": crate.spec(version = "1.3.0"),
    #     "wasm-bindgen-futures": crate.spec(version = "0.4.33"),
    #     "wasm-logger": crate.spec(version = "0.2.0"),
    #     "log": crate.spec(version = "0.4.17"),
    #     "tokio": crate.spec(version = "1.22.0", features = ["full"]),
    #     "warp": crate.spec(version = "0.3.3"),
    #     "clap": crate.spec(version = "3.2.23", features = ["derive"]),
    # },
)

load("@crates//:defs.bzl", "crate_repositories")
crate_repositories()
