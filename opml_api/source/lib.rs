//! This crate provides an API to parse and construct
//! [OPML documents](http://dev.opml.org/spec2.html) to and from regular Rust
//! structs.
//!
//! ## Getting Started
//!
//! ### Parsing
//!
//! Parsing XML into [an OPML struct](struct.OPML.html) can be done with
//! [`OPML::new()`](struct.OPML.html#method.new). Resulting in an error if the
//! XML can't be parsed, if the included OPML version is not supported
//! (currently all OPML versions (1.0, 1.1 and 2.0) are supported) or if the
//! [Body](struct.Body.html) element contains no child
//! [Outline](struct.Outline.html) elements,
//! [as per the spec](http://dev.opml.org/spec2.html#whatIsALtbodygt).
//!
//! ```rust
//! use opml::{OPML, Outline};
//!
//! let xml = r#"<opml version="2.0"><head/><body><outline text="Outline"/></body></opml>"#;
//! let parsed = OPML::new(xml).unwrap();
//!
//! let mut expected = OPML::default();
//! expected.body.outlines.push(Outline {
//!   text: "Outline".to_string(),
//!   ..Outline::default()
//! });
//!
//! println!("{:#?}", parsed);
//! assert_eq!(parsed, expected);
//! ```
//!
//! ### Constructing
//!
//! Constructing OPMLs is very easy as all you have to do is instantiate the
//! [OPML struct](struct.OPML.html) with
//! [`OPML::default()`](struct.OPML.html#method.default), add anything wanted
//! and then call [`OPML::to_xml()`](struct.OPML.html#method.to_xml) to return
//! the XML as a string.
//!
//! ```rust
//! use opml::{Head, OPML};
//!
//! let mut opml = OPML::default();
//! opml.head = Some(Head {
//!   title: Some("Rust Feeds".to_string()),
//!   ..Head::default()
//! });
//! opml
//!   .add_feed("Rust Blog", "https://blog.rust-lang.org/feed.xml")
//!   .add_feed(
//!     "Inside Rust",
//!     "https://blog.rust-lang.org/inside-rust/feed.xml",
//!   );
//!
//! let xml = opml.to_xml().unwrap();
//! let expected = r#"<opml version="2.0"><head><title>Rust Feeds</title></head><body><outline text="Rust Blog" xmlUrl="https://blog.rust-lang.org/feed.xml"/><outline text="Inside Rust" xmlUrl="https://blog.rust-lang.org/inside-rust/feed.xml"/></body></opml>"#;
//! println!("{}", xml);
//! assert_eq!(xml, expected);
//! ```

use regex::Regex;
use serde::{Deserialize, Serialize};
use strong_xml::{XmlRead, XmlWrite};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("OPML body has no <outline> elements")]
  BodyHasNoOutlines,

  #[error("Failed to read file")]
  IoError(#[from] std::io::Error),

  #[error("Unsupported OPML version: {0:?}")]
  UnsupportedVersion(String),

  #[error("Failed to process XML file")]
  XmlError(#[from] strong_xml::XmlError),
}

/// The top-level [OPML](struct.OPML.html) element.
#[derive(
  XmlWrite, XmlRead, PartialEq, Debug, Clone, Serialize, Deserialize,
)]
#[xml(tag = "opml")]
pub struct OPML {
  /// The version attribute from the element, valid values are `1.0`, `1.1` and `2.0`.
  #[xml(attr = "version")]
  pub version: String,

  /// The [Head](struct.Head.html) child element. Contains the metadata of the OPML document.
  #[xml(child = "head")]
  pub head: Option<Head>,

  /// The [Body](struct.Body.html) child element. Contains all the [Outlines](struct.Outline.html).
  #[xml(child = "body")]
  pub body: Body,
}

impl OPML {
  /// Parses an OPML document.
  ///
  /// # Example
  ///
  /// ```rust
  /// use opml::{OPML, Outline};
  ///
  /// let xml = r#"<opml version="2.0"><head/><body><outline text="Outline"/></body></opml>"#;
  /// let parsed = OPML::new(xml).unwrap();
  ///
  /// let mut expected = OPML::default();
  /// expected.body.outlines.push(Outline {
  ///   text: "Outline".to_string(),
  ///   ..Outline::default()
  /// });
  ///
  /// assert_eq!(parsed, expected);
  /// ```
  #[deprecated(note = "use from_str instead", since = "1.1.0")]
  pub fn new(xml: &str) -> Result<Self, Error> {
    Self::from_str(xml).map_err(Into::into)
  }

  /// Parses an OPML document.
  ///
  /// # Example
  ///
  /// ```rust
  /// use opml::{OPML, Outline};
  ///
  /// let xml = r#"<opml version="2.0"><head/><body><outline text="Outline"/></body></opml>"#;
  /// let parsed = OPML::from_str(xml).unwrap();
  ///
  /// let mut expected = OPML::default();
  /// expected.body.outlines.push(Outline {
  ///   text: "Outline".to_string(),
  ///   ..Outline::default()
  /// });
  ///
  /// assert_eq!(parsed, expected);
  /// ```
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(xml: &str) -> Result<Self, Error> {
    let opml = <OPML as XmlRead>::from_str(xml)?;

    let version = &opml.version;

    // SPEC: The version attribute is a version string, of the form, x.y, where x and y are both numeric strings.
    let valid_version_regex = Regex::new(r"^\d+\.\d+$").unwrap();
    let valid_versions = vec!["1.0", "1.1", "2.0"];

    if !valid_version_regex.is_match(version)
      || !valid_versions.contains(&version.as_str())
    {
      return Err(Error::UnsupportedVersion(opml.version));
    }

    // SPEC: A `<body>` contains one or more `<outline>` elements.
    if opml.body.outlines.is_empty() {
      return Err(Error::BodyHasNoOutlines);
    }

    Ok(opml)
  }

  /// Parses an OPML document from a reader.
  ///
  /// # Example
  ///
  /// ```rust,no_run
  /// use opml::{OPML, Outline};
  /// use std::fs::File;
  ///
  /// let mut file = File::open("file.opml").unwrap();
  /// let parsed = OPML::from_reader(&mut file).unwrap();
  /// ```
  pub fn from_reader<R>(reader: &mut R) -> Result<Self, Error>
  where
    R: std::io::Read,
  {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Self::from_str(&s).map_err(Into::into)
  }

  /// Helper function to add an [Outline](struct.Outline.html) element with `text` and `xml_url` attributes to the [Body](struct.Body.html). Useful for creating feed lists quickly. This function [also exists on the Outline struct](struct.Outline.html#method.add_feed) to create grouped lists easily.
  ///
  /// # Example
  ///
  /// ```rust
  /// use opml::{OPML, Outline};
  ///
  /// let mut opml = OPML::default();
  /// opml.add_feed("Feed Name", "https://example.com/");
  /// let added_feed = opml.body.outlines.first().unwrap();
  ///
  /// let expected_feed = &Outline {
  ///   text: "Feed Name".to_string(),
  ///   xml_url: Some("https://example.com/".to_string()),
  ///   ..Outline::default()
  /// };
  ///
  /// assert_eq!(added_feed, expected_feed);
  /// ```
  pub fn add_feed(&mut self, text: &str, url: &str) -> &mut Self {
    self.body.outlines.push(Outline {
      text: text.to_string(),
      xml_url: Some(url.to_string()),
      ..Outline::default()
    });

    self
  }

  /// Converts the struct to an XML document.
  ///
  /// # Example
  ///
  /// ```rust
  /// use opml::OPML;
  ///
  /// let opml = OPML::default();
  /// let xml = opml.to_xml().unwrap();
  ///
  /// let expected = r#"<opml version="2.0"><head/><body/></opml>"#;
  /// assert_eq!(xml, expected);
  /// ```
  #[deprecated(note = "Use to_string instead")]
  pub fn to_xml(&self) -> Result<String, Error> {
    self.to_string()
  }

  /// Converts the struct to an XML document.
  ///
  /// # Example
  ///
  /// ```rust
  /// use opml::OPML;
  ///
  /// let opml = OPML::default();
  /// let xml = opml.to_string().unwrap();
  ///
  /// let expected = r#"<opml version="2.0"><head/><body/></opml>"#;
  /// assert_eq!(xml, expected);
  /// ```
  pub fn to_string(&self) -> Result<String, Error> {
    Ok(XmlWrite::to_string(self)?)
  }

  /// Converts the struct to an XML document and writes it using the writer.
  ///
  /// # Example
  ///
  /// ```rust,no_run
  /// use opml::OPML;
  /// use std::fs::File;
  ///
  /// let opml = OPML::default();
  /// let mut file = File::create("file.opml").unwrap();
  /// let xml = opml.to_writer(&mut file).unwrap();
  /// ```
  pub fn to_writer<W>(&self, writer: &mut W) -> Result<(), Error>
  where
    W: std::io::Write,
  {
    let xml_string = self.to_string()?;
    writer.write_all(&xml_string.as_bytes())?;
    Ok(())
  }
}

impl Default for OPML {
  fn default() -> Self {
    OPML {
      version: "2.0".to_string(),
      head: Some(Head::default()),
      body: Body::default(),
    }
  }
}

/// The [Head](struct.Head.html) child element of [OPML](struct.OPML.html).
/// Contains the metadata of the OPML document.
#[derive(
  XmlWrite, XmlRead, PartialEq, Debug, Clone, Default, Serialize, Deserialize,
)]
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

/// The [Body](struct.Body.html) child element of [OPML](struct.OPML.html). Contains all the [Outlines](struct.Outline.html).
#[derive(
  XmlWrite, XmlRead, PartialEq, Debug, Clone, Default, Serialize, Deserialize,
)]
#[xml(tag = "body")]
pub struct Body {
  /// All the top-level [Outline](struct.Outline.html) elements.
  #[xml(child = "outline")]
  pub outlines: Vec<Outline>,
}

/// The [Outline](struct.Outline.html) element.
#[derive(
  XmlWrite, XmlRead, PartialEq, Debug, Clone, Default, Serialize, Deserialize,
)]
#[xml(tag = "outline")]
pub struct Outline {
  /// Every outline element must have at least a text attribute, which is what is displayed when an outliner opens the OPML document.
  /// Version 1.0 OPML documents may omit this attribute, so for compatibility and strictness this attribute is "technically optional" as it will be replaced by an empty String if it is omitted.
  /// Text attributes may contain encoded HTML markup.
  #[xml(default, attr = "text")]
  pub text: String,

  /// A string that indicates how the other attributes of the [Outline](struct.Outline.html) should be interpreted.
  #[xml(attr = "type")]
  pub r#type: Option<String>,

  /// Indicating whether the outline is commented or not. By convention if an outline is commented, all subordinate outlines are considered to also be commented.
  #[xml(attr = "isComment")]
  pub is_comment: Option<bool>,

  /// Indicating whether a breakpoint is set on this outline. This attribute is mainly necessary for outlines used to edit scripts.
  #[xml(attr = "isBreakpoint")]
  pub is_breakpoint: Option<bool>,

  /// The date-time (RFC822) that this [Outline](struct.Outline.html) element was created.
  #[xml(attr = "created")]
  pub created: Option<String>,

  /// A string of comma-separated slash-delimited category strings, in the format defined by the [RSS 2.0 category](http://cyber.law.harvard.edu/rss/rss.html#ltcategorygtSubelementOfLtitemgt) element. To represent a "tag," the category string should contain no slashes.
  #[xml(attr = "category")]
  pub category: Option<String>,

  /// Child [Outline](struct.Outline.html) elements of the current one.
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

  /// A link that can point to another OPML document or to something that can be displayed in a web browser.
  #[xml(attr = "url")]
  pub url: Option<String>,
}

impl Outline {
  /// Helper function to add an [Outline](struct.Outline.html) element with `text` and `xml_url` attributes as a child element. Useful for creating grouped lists. This function [also exists on the OPML struct](struct.OPML.html#method.add_feed) for non-grouped lists.
  ///
  /// # Example
  ///
  /// ```rust
  /// use opml::{Outline};
  ///
  /// let mut group = Outline::default();
  /// group.add_feed("Feed Name", "https://example.com/");
  /// let added_feed = group.outlines.first().unwrap();
  ///
  /// let expected_feed = &Outline {
  ///   text: "Feed Name".to_string(),
  ///   xml_url: Some("https://example.com/".to_string()),
  ///   ..Outline::default()
  /// };
  ///
  /// assert_eq!(added_feed, expected_feed);
  /// ```
  pub fn add_feed(&mut self, name: &str, url: &str) -> &mut Self {
    self.outlines.push(Outline {
      text: name.to_string(),
      xml_url: Some(url.to_string()),
      ..Outline::default()
    });

    self
  }
}
