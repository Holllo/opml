//! This crate provides an API to parse and construct [OPML files](https://dev.opml.org/spec2.html) to and from regular Rust structs.
//!
//! ## Getting Started
//!
//! ### Parsing
//!
//! ```rust
//! use opml::OPML;
//!
//! let xml = r#"<opml version="2.0"><head/><body><outline text="Outline"/></body></opml>"#;
//! let parsed = OPML::new(xml).unwrap();
//!
//! println!("{:#?}", parsed);
//! ```
//!
//! ### Constructing
//!
//! ```rust
//! use opml::OPML;
//!
//! let mut opml = OPML::default();
//! opml
//!   .add_feed("Rust Blog", "https://blog.rust-lang.org/feed.xml")
//!   .add_feed(
//!     "Inside Rust",
//!     "https://blog.rust-lang.org/inside-rust/feed.xml",
//!   );
//!
//! opml.head.title = Some("Rust Feeds".to_string());
//!
//! let xml = opml.to_xml().unwrap();
//! println!("{}", xml);
//!
//! // Outputs (without whitespace):
//! // <opml version="2.0">
//! //   <head>
//! //     <title>Rust Feeds</title>
//! //   </head>
//! //   <body>
//! //     <outline text="Rust Blog" xmlUrl="https://blog.rust-lang.org/feed.xml"/>
//! //     <outline text="Inside Rust" xmlUrl="https://blog.rust-lang.org/inside-rust/feed.xml"/>
//! //   </body>
//! // </opml>
//! ```
//!
//! ## License
//!
//! Open-sourced with either the
//!
//! * [Apache License, Version 2.0](https://gitlab.com/holllo/opml-rs/-/blob/master/License-Apache) (http://www.apache.org/licenses/LICENSE-2.0)
//! * [MIT license](https://gitlab.com/holllo/opml-rs/-/blob/master/License-MIT) (http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! The samples [located in `tests/spec_samples`](https://gitlab.com/holllo/opml-rs/-/blob/master/tests/spec_samples) were [taken from the OPML 2.0 spec](http://dev.opml.org/spec2.html#examples) and are subject to [their own license](https://gitlab.com/holllo/opml-rs/-/blob/master/tests/spec_samples/License).

// TODO: Maybe use a date-time type for all the date-time places?

use regex::Regex;
use strong_xml::{XmlError, XmlRead, XmlWrite};

/// The top-level `<opml>` element.
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "opml")]
pub struct OPML {
  /// The version attribute from the element.
  #[xml(attr = "version")]
  pub version: String,

  /// The `<head>` child element. Contains the metadata of the OPML document.
  #[xml(child = "head")]
  pub head: Head,

  /// The `<body>` child element. Contains all the `<outlines>`.
  #[xml(child = "body")]
  pub body: Body,
}

impl OPML {
  /// Parses an OPML file.
  pub fn new(xml: &str) -> Result<Self, String> {
    let opml: Result<OPML, XmlError> = OPML::from_str(xml);

    let opml = match opml {
      Ok(value) => value,
      Err(err) => return Err(format!("XML parsing error: {:#?}", err)),
    };

    // TODO: Maybe implement version 1.0 and 1.1 of the OPML spec?
    // SPEC: The version attribute is a version string, of the form, x.y, where x and y are both numeric strings.
    let valid_version_regex = Regex::new(r"^\d+\.\d+$").unwrap();

    if !valid_version_regex.is_match(opml.version.as_str())
      || opml.version != "2.0"
    {
      return Err(format!(
        "Unsupported OPML version detected: {}",
        opml.version
      ));
    }

    // SPEC: A `<body>` contains one or more `<outline>` elements.
    if opml.body.outlines.is_empty() {
      return Err("OPML body has no outlines.".to_string());
    }

    Ok(opml)
  }

  pub fn add_feed(&mut self, name: &str, url: &str) -> &mut Self {
    self.body.outlines.push(Outline {
      text: name.to_string(),
      xml_url: Some(url.to_string()),
      ..Outline::default()
    });

    self
  }

  pub fn to_xml(&self) -> Result<String, String> {
    let result: Result<String, XmlError> = self.to_string();

    match result {
      Ok(value) => Ok(value),
      Err(err) => Err(format!("XML writing error: {:#?}", err)),
    }
  }
}

impl Default for OPML {
  fn default() -> Self {
    OPML {
      version: "2.0".to_string(),
      head: Head::default(),
      body: Body::default(),
    }
  }
}

/// The `<head>` child element of `<opml>`.
/// Contains the metadata of the OPML document.
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone, Default)]
#[xml(tag = "head")]
pub struct Head {
  /// The title of the document.
  #[xml(flatten_text = "title")]
  pub title: Option<String>,

  /// A date-time (RFC822) indicating when the document was created.
  #[xml(flatten_text = "dateCreated")]
  pub date_created: Option<String>,

  /// A date-time (RFC822) indicating when the document was last modified.
  #[xml(flatten_text = "dateModified")]
  pub date_modified: Option<String>,

  /// The name of the document owner.
  #[xml(flatten_text = "ownerName")]
  pub owner_name: Option<String>,

  /// The email address of the document owner.
  #[xml(flatten_text = "ownerEmail")]
  pub owner_email: Option<String>,

  /// A link to the website of the document owner.
  #[xml(flatten_text = "ownerId")]
  pub owner_id: Option<String>,

  /// A link to the documentation of the OPML format used for this document.
  #[xml(flatten_text = "docs")]
  pub docs: Option<String>,

  /// A comma-separated list of line numbers that are expanded. The line numbers in the list tell you which headlines to expand. The order is important. For each element in the list, X, starting at the first summit, navigate flatdown X times and expand. Repeat for each element in the list.
  #[xml(flatten_text = "expansionState")]
  pub expansion_state: Option<String>,

  /// A number indicating which line of the outline is displayed on the top line of the window. This number is calculated with the expansion state already applied.
  #[xml(flatten_text = "vertScrollState")]
  pub vert_scroll_state: Option<i32>,

  /// The pixel location of the top edge of the window.
  #[xml(flatten_text = "windowTop")]
  pub window_top: Option<i32>,

  /// The pixel location of the left edge of the window.
  #[xml(flatten_text = "windowLeft")]
  pub window_left: Option<i32>,

  /// The pixel location of the bottom edge of the window.
  #[xml(flatten_text = "windowBottom")]
  pub window_bottom: Option<i32>,

  /// The pixel location of the right edge of the window.
  #[xml(flatten_text = "windowRight")]
  pub window_right: Option<i32>,
}

/// The `<body>` child element of `<opml>`. Contains all the `<outlines>`.
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone, Default)]
#[xml(tag = "body")]
pub struct Body {
  /// `<outline>` elements.
  #[xml(child = "outline")]
  pub outlines: Vec<Outline>,
}

/// The `<outline>` element.
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone, Default)]
#[xml(tag = "outline")]
pub struct Outline {
  /// Every outline element must have at least a text attribute, which is what is displayed when an outliner opens the OPML file.
  /// Text attributes may contain encoded HTML markup.
  #[xml(attr = "text")]
  pub text: String,

  /// A string that indicates how the other attributes of the `<outline>` should be interpreted.
  #[xml(attr = "type")]
  pub r#type: Option<String>,

  /// Indicating whether the outline is commented or not. By convention if an outline is commented, all subordinate outlines are considered to also be commented.
  #[xml(attr = "isComment")]
  pub is_comment: Option<bool>,

  /// Indicating whether a breakpoint is set on this outline. This attribute is mainly necessary for outlines used to edit scripts.
  #[xml(attr = "isBreakpoint")]
  pub is_breakpoint: Option<bool>,

  /// The date-time (RFC822) that this `<outline>` element was created.
  #[xml(attr = "created")]
  pub created: Option<String>,

  /// A string of comma-separated slash-delimited category strings, in the format defined by the [RSS 2.0 category](http://cyber.law.harvard.edu/rss/rss.html#ltcategorygtSubelementOfLtitemgt) element. To represent a "tag," the category string should contain no slashes.
  #[xml(attr = "category")]
  pub category: Option<String>,

  /// Child `<outline>` elements of the current one.
  #[xml(child = "outline")]
  pub outlines: Vec<Outline>,

  /// The HTTP address of the feed.
  #[xml(attr = "xmlUrl")]
  pub xml_url: Option<String>,

  /// The top-level description element from the feed.
  #[xml(attr = "description")]
  pub description: Option<String>,

  /// The top-level link element from the feed.
  #[xml(attr = "htmlUrl")]
  pub html_url: Option<String>,

  /// The top-level language element from the feed.
  #[xml(attr = "language")]
  pub language: Option<String>,

  /// The top-level title element from the feed.
  #[xml(attr = "title")]
  pub title: Option<String>,

  /// The version of the feed's format (such as RSS 0.91, 2.0, ...).
  #[xml(attr = "version")]
  pub version: Option<String>,

  /// A link that can point to another OPML file or to something that can be displayed in a web browser.
  #[xml(attr = "url")]
  pub url: Option<String>,
}

impl Outline {
  pub fn add_feed(&mut self, name: &str, url: &str) -> &mut Self {
    self.outlines.push(Outline {
      text: name.to_string(),
      xml_url: Some(url.to_string()),
      ..Outline::default()
    });

    self
  }
}
