load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

def cargo_dependencies():
    crates_repository(
        name = "crates",
        cargo_lockfile = "//:Cargo.bzl.lock",
        isolated = False,
        packages = {
            "yew": crate.spec(version = "0.20.0", features = ["ssr", "hydration"]),
			"reqwest": crate.spec(version = "0.11.13", features = ["json"]),
            "serde": crate.spec(version = "1.0.148", features = ["derive"]),
            "uuid": crate.spec(version = "1.2.2", features = ["serde"]),
            "futures": crate.spec(version = "0.3.25"),
            "bytes": crate.spec(version = "1.3.0"),

            "wasm-bindgen-futures": crate.spec(version = "0.4.33"),
            "wasm-logger": crate.spec(version = "0.2.0"),
            "log": crate.spec(version = "0.4.17"),

            "tokio": crate.spec(version = "1.22.0", features = ["full"]),
            "warp": crate.spec(version = "0.3.3"),
            "clap": crate.spec(version = "3.2.23", features = ["derive"]),
        },
    )
