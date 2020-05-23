// TODO: Maybe use a date-time type for all the date-time places?

use regex::Regex;
use strong_xml::{XmlError, XmlRead, XmlWrite};

/// `<opml>` is an XML element, with a single required attribute, version; a `<head>` element and a `<body>` element, both of which are required.
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "opml")]
pub struct OPML {
  /// The version attribute is a version string, of the form, x.y, where x and y are both numeric strings.
  #[xml(attr = "version")]
  pub version: String,

  /// A `<head>` contains zero or more optional elements.
  #[xml(child = "head")]
  pub head: Head,

  /// A `<body>` contains one or more `<outline>` elements.
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
}

/// A `<head>` contains zero or more optional elements.
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
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

  /// The owner of the document.
  #[xml(flatten_text = "ownerName")]
  pub owner_name: Option<String>,

  /// The email address of the owner of the document.
  #[xml(flatten_text = "ownerEmail")]
  pub owner_email: Option<String>,

  /// A link to the website of the owner of the document.
  #[xml(flatten_text = "ownerId")]
  pub owner_id: Option<String>,

  /// A link to the documentation of the OPML format.
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

/// A `<body>` contains one or more `<outline>` elements.
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "body")]
pub struct Body {
  /// Child outlines of the body.
  #[xml(child = "outline")]
  pub outlines: Vec<Outline>,
}

/// An `<outline>` is an XML element containing at least one required attribute, text, and zero or more additional attributes. An `<outline>` may contain zero or more `<outline>` sub-elements. No attribute may be repeated within the same `<outline>` element.
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "outline")]
pub struct Outline {
  /// Every outline element must have at least a text attribute, which is what is displayed when an outliner opens the OPML file.
  /// Text attributes may contain encoded HTML markup.
  #[xml(attr = "text")]
  pub text: String,

  /// A string that says how the other attributes of the `<outline>` are interpreted.
  #[xml(attr = "type")]
  pub r#type: Option<String>,

  /// Indicating whether the outline is commented or not. By convention if an outline is commented, all subordinate outlines are considered to also be commented. If it's not present, the value is false.
  #[xml(default, attr = "isComment")]
  pub is_comment: bool,

  /// Indicating whether a breakpoint is set on this outline. This attribute is mainly necessary for outlines used to edit scripts. If it's not present, the value is false.
  #[xml(default, attr = "isBreakpoint")]
  pub is_breakpoint: bool,

  /// The date-time (RFC822) that this outline node was created.
  #[xml(attr = "created")]
  pub created: Option<String>,

  /// A string of comma-separated slash-delimited category strings, in the format defined by the [RSS 2.0 category](http://cyber.law.harvard.edu/rss/rss.html#ltcategorygtSubelementOfLtitemgt) element. To represent a "tag," the category string should contain no slashes.
  #[xml(attr = "category")]
  pub category: Option<String>,

  /// Child outlines of the current one.
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
