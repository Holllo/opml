use std::fs::read_to_string as read;

use opml::*;

#[test]
#[should_panic]
fn test_invalid_xml() {
  let sample = read("tests/samples/invalid_xml.opml").unwrap();
  OPML::new(sample.as_str()).unwrap();
}

#[test]
#[should_panic(expected = "Unsupported OPML version detected: invalid")]
fn test_invalid_opml_version() {
  let sample = read("tests/samples/invalid_opml_version.opml").unwrap();
  OPML::new(sample.as_str()).unwrap();
}

#[test]
#[should_panic(expected = "OPML body has no outlines.")]
fn test_invalid_opml_no_outlines() {
  let sample = read("tests/samples/invalid_opml_no_outlines.opml").unwrap();
  OPML::new(sample.as_str()).unwrap();
}
