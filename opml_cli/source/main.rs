use std::fs::read_to_string;

use clap::{
  crate_authors, crate_description, crate_version, App, Arg, ArgGroup,
};
use opml::{Outline, OPML};

fn main() {
  let cli = App::new("OPML CLI")
    .about(crate_description!())
    .author(crate_authors!())
    .version(crate_version!())
    .args(&[
      // Format flags.
      Arg::with_name("json")
        .long("json")
        .long_help("Output the OPML as JSON.")
        .takes_value(false),
      Arg::with_name("json pretty")
        .long("json-pretty")
        .long_help("Output the OPML as pretty-printed JSON.")
        .takes_value(false),
      Arg::with_name("rss")
        .long("rss")
        .long_help(
          "Only output the outline text and xmlUrl attributes \
          when both are present in the outline element.",
        )
        .takes_value(false),
      // Boolean flags.
      Arg::with_name("verbose")
        .long("verbose")
        .long_help("Print extra information while running.")
        .takes_value(false),
      // Options that are only allowed once.
      Arg::with_name("file")
        .long("file")
        .long_help("The OPML file to parse.")
        .required(true)
        .short("f")
        .takes_value(true),
    ])
    .group(
      ArgGroup::with_name("format")
        .args(&["json", "json pretty", "rss"])
        .required(true),
    )
    .get_matches();

  // Extract format flags.
  let json = cli.is_present("json");
  let json_pretty = cli.is_present("json pretty");
  let rss = cli.is_present("rss");

  // Extract boolean flags.
  let verbose = cli.is_present("verbose");

  // Extract the various options.
  let file = cli.value_of("file").unwrap();

  // Read the file to string.
  let xml = read_to_string(file).expect("Failed to read OPML file");

  // Parse the OPML from the read file.
  let opml = OPML::from_str(&xml).expect("Failed to parse OPML file");

  if rss {
    // Get all the outlines from the OPML document.
    let outlines = extract_all_outlines(&opml.body.outlines);

    // Print out the text and xmlUrl attributes when possible.
    for outline in outlines {
      if let Some(xml_url) = outline.xml_url {
        println!("{}", outline.text);
        println!("{}", xml_url);
      } else if verbose {
        println!(
          "Skipping \"{}\" because it did not have an xmlUrl attribute.",
          outline.text
        );
      }
    }
  } else if json {
    println!(
      "{}",
      serde_json::to_string(&opml).expect("Failed to convert OPML to JSON")
    );
  } else if json_pretty {
    println!(
      "{}",
      serde_json::to_string_pretty(&opml)
        .expect("Failed to convert OPML to pretty JSON")
    );
  } else {
    unreachable!();
  }
}

/// A helper function that takes in `opml::Outline` elements and returns all
/// children it can find in a single `Vec<Outline>`.
pub fn extract_all_outlines(outlines: &[Outline]) -> Vec<Outline> {
  let mut accumulator = vec![];

  for outline in outlines {
    accumulator.push(outline.clone());
    accumulator.append(&mut extract_all_outlines(&outline.outlines));
  }

  accumulator
}
