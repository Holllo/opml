use std::error::Error;
use std::fs::read_to_string as read;

use opml::*;

#[test]
fn test_opml_construction_1() -> Result<(), Box<dyn Error>> {
  let mut opml = OPML::default();
  opml
    .add_feed("Rust Blog", "https://blog.rust-lang.org/feed.xml")
    .add_feed(
      "Inside Rust",
      "https://blog.rust-lang.org/inside-rust/feed.xml",
    );
  opml.head = Some(Head {
    title: Some("Rust Feeds".to_string()),
    ..Head::default()
  });

  let actual = opml.to_xml().unwrap();
  let expected = read("tests/samples/construction_1.opml")?;

  assert_eq!(actual.trim(), expected.trim());

  Ok(())
}

#[test]
fn test_opml_construction_2() -> Result<(), Box<dyn Error>> {
  let mut opml = OPML::default();
  opml.head = Some(Head {
    title: Some("Rust Feeds".to_string()),
    ..Head::default()
  });

  let mut rust_group = Outline::default();
  rust_group.text = "Rust Feeds".to_string();
  rust_group
    .add_feed("Rust Blog", "https://blog.rust-lang.org/feed.xml")
    .add_feed(
      "Inside Rust",
      "https://blog.rust-lang.org/inside-rust/feed.xml",
    );

  let mut mozilla_group = Outline::default();
  mozilla_group.text = "Mozilla Feeds".to_string();
  mozilla_group
    .add_feed("Mozilla Blog", "https://blog.mozilla.org/feed")
    .add_feed("Mozilla Hacks", "https://hacks.mozilla.org/feed");

  opml.body.outlines.push(rust_group);
  opml.body.outlines.push(mozilla_group);

  let actual = opml.to_xml().unwrap();
  let expected = read("tests/samples/construction_2.opml")?;

  assert_eq!(actual.trim(), expected.trim());

  Ok(())
}
