use std::{error::Error, fs};

use opml::*;

#[test]
pub fn test_spec_samples() -> Result<(), Box<dyn Error>> {
  let samples = vec![
    "tests/spec_samples/category.opml",
    "tests/spec_samples/directory.opml",
    "tests/spec_samples/placesLived.opml",
    "tests/spec_samples/simpleScript.opml",
    "tests/spec_samples/states.opml",
    "tests/spec_samples/subscriptionList.opml",
  ];

  for sample in samples {
    let sample_content = fs::read_to_string(&sample)?;
    OPML::from_str(&sample_content)?;
  }

  Ok(())
}
