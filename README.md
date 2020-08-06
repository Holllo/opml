# OPML

> An OPML parser for Rust.

## Getting Started

```rust
use opml::OPML;

let xml = r#"<opml version="2.0"><body><outline text="Outline"/></body></opml>"#;
let parsed = OPML::new(xml).unwrap();

println!("{:#?}", parsed);
```

Check out [the documentation](https://docs.rs/opml/) for further details.

## CLI

Looking for a command-line utility to parse OPML documents? Check out [the `opml-cli`](https://git.holllo.cc/Holllo/opml-cli/), a small wrapper around this crate that will let you do just that.

## License

Open-sourced with either the

* [Apache License, Version 2.0](https://git.holllo.cc/Holllo/opml/src/branch/main/LICENSE-Apache) (http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](https://git.holllo.cc/Holllo/opml/src/branch/main/LICENSE-MIT) (http://opensource.org/licenses/MIT)

at your option.

The samples [located in `tests/spec_samples`](https://git.holllo.cc/Holllo/opml/src/branch/main/tests/spec_samples) were [taken from the OPML 2.0 spec](http://dev.opml.org/spec2.html#examples) and are subject to [their own license](https://git.holllo.cc/Holllo/opml/src/branch/main/tests/spec_samples/LICENSE).
