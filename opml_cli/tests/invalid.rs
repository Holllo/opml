use std::fs::read_to_string;

use assert_cmd::Command;

const SAMPLE: &str = "tests/samples/youtube.opml";

#[test]
fn test_missing_file() {
  let mut cmd = Command::cargo_bin("opml").unwrap();
  let assert = cmd.args(&["--rss"]).assert();

  assert
    .failure()
    .code(1)
    .stderr(read_to_string("tests/snapshots/missing-file.txt").unwrap());
}

#[test]
fn test_missing_format() {
  let mut cmd = Command::cargo_bin("opml").unwrap();
  let assert = cmd.args(&["--file", SAMPLE]).assert();

  assert
    .failure()
    .code(1)
    .stderr(read_to_string("tests/snapshots/missing-format.txt").unwrap());
}
