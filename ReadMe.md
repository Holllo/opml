# OPML

> An OPML parser for Rust.

## Contents

* [Examples](examples/)
* [Expertise Wanted](#expertise-wanted)
* [License](#license)

## Expertise Wanted

This is the first crate I'm publishing to crates.io and the first proper library I've made for Rust. Any help or tips with how to improve the API, documentation, usability, testing or anything else is very welcome. Feel free to create [an issue](https://gitlab.com/holllo/opml-rs/issues/) or contact me via email at helllo@holllo.cc.

## Getting Started

```rust
use opml::OPML;

let xml = r#"<opml version="2.0"><head/><body><outline text="Outline"/></body></opml>"#;
let parsed = OPML::new(xml).unwrap();

println!("{:#?}", parsed);
```

## License

Open-sourced with either the

* [Apache License, Version 2.0](License-Apache) (http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](License-MIT) (http://opensource.org/licenses/MIT)

at your option.

The samples [located in `tests/spec_samples`](tests/spec_samples) were [taken from the OPML 2.0 spec](http://dev.opml.org/spec2.html#examples) and are subject to [their own license](tests/spec_samples/License).
