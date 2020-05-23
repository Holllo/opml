// TODO: Extract all the OPML strings out and put them into their own files.

use opml::*;

#[test]
#[should_panic]
fn test_invalid_xml() {
  OPML::new(r#"{not xml :)"#).unwrap();
}

#[test]
#[should_panic(expected = "Unsupported OPML version detected: 1.0")]
fn test_invalid_opml_version_1_0() {
  OPML::new(
    r#"
<opml version="1.0">
<head/>
<body>
  <outline text="Outline Text"/>
</body>
</opml>"#,
  )
  .unwrap();
}

#[test]
#[should_panic(expected = "Unsupported OPML version detected: 1.1")]
fn test_invalid_opml_version_1_1() {
  OPML::new(
    r#"
<opml version="1.1">
<head/>
<body>
  <outline text="Outline Text"/>
</body>
</opml>"#,
  )
  .unwrap();
}

#[test]
#[should_panic(expected = "Unsupported OPML version detected: invalid")]
fn test_invalid_opml_version() {
  OPML::new(
    r#"
<opml version="invalid">
<head/>
<body>
  <outline text="Outline Text"/>
</body>
</opml>"#,
  )
  .unwrap();
}

#[test]
#[should_panic(expected = "OPML body has no outlines.")]
fn test_invalid_opml_no_outlines() {
  OPML::new(r#"<opml version="2.0"><head/><body/></opml>"#).unwrap();
}
