# OPML 📄

> **OPML library for Rust & standalone CLI.**

## API

For API documentation and examples see [docs.rs](https://docs.rs/opml).

## CLI

### Cargo

With a working [Rust and Cargo](https://www.rust-lang.org/learn/get-started) installation, you can install the OPML CLI from [Crates.io](https://crates.io/crates/opml_cli).

```
cargo install opml_cli
```

### Binaries

Precompiled `x86_64-unknown-linux-gnu` binaries are available on the [Releases page](https://git.bauke.xyz/Holllo/opml/releases).

## Development

With [Nix flakes](https://nixos.wiki/wiki/Flakes) and [direnv](https://direnv.net/) installed and enabled, all the required dependencies are automatically loaded from [`shell.nix`](./shell.nix). Then [cargo-make](https://sagiegurari.github.io/cargo-make/) can be used to build, deploy and lint the code. The available tasks are all described in the [`Makefile.toml`](Makefile.toml) configuration.

## License

Distributed under the [Apache License 2.0](https://spdx.org/licenses/Apache-2.0.html) and [MIT](https://spdx.org/licenses/MIT.html) licenses, see [LICENSE-Apache](https://git.bauke.xyz/Holllo/opml/src/branch/main/LICENSE-Apache) and [LICENSE-MIT](https://git.bauke.xyz/Holllo/opml/src/branch/main/LICENSE-MIT) for more information.

The samples located in `opml_api/tests/spec_samples`) were [taken from the OPML 2.0 spec](http://dev.opml.org/spec2.html#examples) and are subject to [their own license](https://git.bauke.xyz/Holllo/opml/src/branch/main/opml_api/tests/spec_samples/LICENSE).
