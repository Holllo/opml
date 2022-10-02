use opml::OPML;

const SAMPLE: &str = r#"<opml version="2.0">
<head>
  <title>Rust Feeds</title>
</head>
<body>
  <outline text="Rust Blog" xmlUrl="https://blog.rust-lang.org/feed.xml" />
  <outline text="Inside Rust" xmlUrl="https://blog.rust-lang.org/inside-rust/feed.xml" />
</body>
</opml>"#;

/// Run this example using `cargo run --example rss`.
///
/// Output:
/// Rust Feeds
/// ──────────
/// Rust Blog      https://blog.rust-lang.org/feed.xml
/// Inside Rust    https://blog.rust-lang.org/inside-rust/feed.xml
fn main() {
  let subscriptions = OPML::from_str(SAMPLE).unwrap();

  if let Some(title) = subscriptions.head.and_then(|head| head.title) {
    println!("{}", title);
    println!("{}", "-".repeat(title.len()));
  }

  for outline in subscriptions.body.outlines {
    println!("{}\t{}", outline.text, outline.xml_url.unwrap());
  }
}
