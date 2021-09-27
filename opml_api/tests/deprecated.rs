use opml::OPML;

#[test]
#[allow(deprecated)]
fn test_deprecated_functions() {
  let xml = r#"<opml version="2.0"><head/><body><outline text="Outline"/></body></opml>"#;
  let document = OPML::new(xml).unwrap();

  assert_eq!(document.version, "2.0");
  assert!(document.to_xml().is_ok());
}
