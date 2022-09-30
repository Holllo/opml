use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use opml::{Outline, OPML};

#[derive(Debug, Parser)]
#[clap(about, author, version)]
struct Args {
  /// The OPML file to parse.
  #[clap(short, long)]
  file: PathBuf,

  /// Output the OPML as JSON.
  #[clap(long, group = "format", required = true)]
  json: bool,

  /// Output the OPML as pretty-printed JSON.
  #[clap(long, group = "format", required = true)]
  json_pretty: bool,

  /// Only output the outline text and xmlUrl attributes when both are present
  /// in the outline element.
  #[clap(long, group = "format", required = true)]
  rss: bool,

  /// Print extra information while running.
  #[clap(long)]
  verbose: bool,
}

fn main() {
  let args = Args::parse();

  // Read the file to string.
  let xml = read_to_string(args.file).expect("Failed to read OPML file");

  // Parse the OPML from the read file.
  let opml = OPML::from_str(&xml).expect("Failed to parse OPML file");

  if args.rss {
    // Get all the outlines from the OPML document.
    let outlines = extract_all_outlines(&opml.body.outlines);

    // Print out the text and xmlUrl attributes when possible.
    for outline in outlines {
      if let Some(xml_url) = outline.xml_url {
        println!("{}", outline.text);
        println!("{}", xml_url);
      } else if args.verbose {
        println!(
          "Skipping \"{}\" because it did not have an xmlUrl attribute.",
          outline.text
        );
      }
    }
  } else if args.json {
    println!(
      "{}",
      serde_json::to_string(&opml).expect("Failed to convert OPML to JSON")
    );
  } else if args.json_pretty {
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
