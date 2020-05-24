use std::fs::read_to_string as read;

use opml::*;

#[test]
fn test_minimum_valid_opml() {
  assert_eq!(
    OPML::new(&read("tests/samples/minimum_valid_opml.opml").unwrap()).unwrap(),
    OPML {
      version: "2.0".to_string(),
      head: Head {
        title: None,
        date_created: None,
        date_modified: None,
        owner_name: None,
        owner_email: None,
        owner_id: None,
        docs: None,
        expansion_state: None,
        vert_scroll_state: None,
        window_top: None,
        window_left: None,
        window_bottom: None,
        window_right: None,
      },
      body: Body {
        outlines: vec![Outline {
          text: "Outline Text".to_string(),
          r#type: None,
          is_breakpoint: false,
          is_comment: false,
          created: None,
          category: None,
          xml_url: None,
          description: None,
          html_url: None,
          language: None,
          title: None,
          version: None,
          url: None,
          outlines: vec![]
        }]
      },
    }
  );
}

#[test]
fn test_valid_opml_with_everything() {
  assert_eq!(
    OPML::new(&read("tests/samples/valid_opml_with_everything.opml").unwrap())
      .unwrap(),
    OPML {
      version: "2.0".to_string(),
      head: Head {
        title: Some("Title".to_string()),
        date_created: Some("Date Created".to_string()),
        date_modified: Some("Date Modified".to_string()),
        owner_name: Some("Owner Name".to_string()),
        owner_email: Some("Owner Email".to_string()),
        owner_id: Some("Owner ID".to_string()),
        docs: Some("http://dev.opml.org/spec2.html".to_string()),
        expansion_state: Some("0,1".to_string()),
        vert_scroll_state: Some(0),
        window_top: Some(1),
        window_left: Some(2),
        window_bottom: Some(3),
        window_right: Some(4),
      },
      body: Body {
        outlines: vec![Outline {
          text: "Outline Text".to_string(),
          r#type: Some("Outline Type".to_string()),
          is_breakpoint: true,
          is_comment: true,
          created: Some("Outline Date".to_string()),
          category: Some("Outline Category".to_string()),
          xml_url: Some("Outline XML URL".to_string()),
          description: Some("Outline Description".to_string()),
          html_url: Some("Outline HTML URL".to_string()),
          language: Some("Outline Language".to_string()),
          title: Some("Outline Title".to_string()),
          version: Some("Outline Version".to_string()),
          url: Some("Outline URL".to_string()),
          outlines: vec![Outline {
            text: "Nested Outline Text".to_string(),
            r#type: Some("Nested Outline Type".to_string()),
            is_breakpoint: true,
            is_comment: false,
            created: Some("Nested Outline Date".to_string()),
            category: Some("Nested Outline Category".to_string()),
            xml_url: Some("Nested Outline XML URL".to_string()),
            description: Some("Nested Outline Description".to_string()),
            html_url: Some("Nested Outline HTML URL".to_string()),
            language: Some("Nested Outline Language".to_string()),
            title: Some("Nested Outline Title".to_string()),
            version: Some("Nested Outline Version".to_string()),
            url: Some("Nested Outline URL".to_string()),
            outlines: vec![]
          }]
        }]
      },
    }
  )
}
