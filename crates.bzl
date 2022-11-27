load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

def cargo_dependencies():
    crates_repository(
        name = "crates",
        cargo_lockfile = "//:Cargo.lock",
        isolated = False,
        packages = {
            "yew": crate.spec(version = "0.20.0", features = ["ssr", "hydration"]),
			"reqwest": crate.spec(version = "0.11.8", features = ["json"]),
            "serde": crate.spec(version = "1.0.132", features = ["derive"]),
            "uuid": crate.spec(version = "1.0.0", features = ["serde", "js"]),
            "futures": crate.spec(version = "0.3"),
            "bytes": crate.spec(version = "1.0"),

            "wasm-bindgen-futures": crate.spec(version = "0.4"),
            "wasm-logger": crate.spec(version = "0.2"),
            "log": crate.spec(version = "0.4"),

            "tokio": crate.spec(version = "1.15.0", features = ["full"]),
            "warp": crate.spec(version = "0.3"),
            "clap": crate.spec(version = "3.1.7", features = ["derive"]),
        },
    )
