use std::{error::Error, fs};

use opml::OPML;

fn main() -> Result<(), Box<dyn Error>> {
  let xml = fs::read_to_string("examples/opml_samples/rust_feeds.opml")?;

  let subscriptions = OPML::from_str(&xml)?;
  let head = subscriptions.head.unwrap();
  let title = head.title.unwrap();

  println!(" {}", title);
  println!(" {}", "─".repeat(title.len()));

  for outline in subscriptions.body.outlines {
    println!(" {}\t{}", outline.text, outline.xml_url.unwrap());
  }

  Ok(())
}

// Output:
// Rust Feeds
// ──────────
// Rust Blog      https://blog.rust-lang.org/feed.xml
// Inside Rust    https://blog.rust-lang.org/inside-rust/feed.xml
