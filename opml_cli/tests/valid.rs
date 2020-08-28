use std::fs::read_to_string;

use assert_cmd::Command;

const SAMPLE: &str = "tests/samples/youtube.opml";

#[test]
fn test_valid_rss() {
  let mut cmd = Command::cargo_bin("opml").unwrap();
  let assert = cmd.args(&["--file", SAMPLE, "--rss"]).assert();

  assert
    .success()
    .code(0)
    .stdout(read_to_string("tests/snapshots/rss.txt").unwrap());
}

#[test]
fn test_valid_json() {
  let mut cmd = Command::cargo_bin("opml").unwrap();
  let assert = cmd.args(&["--file", SAMPLE, "--json"]).assert();

  assert
    .success()
    .code(0)
    .stdout(read_to_string("tests/snapshots/json.json").unwrap());
}

#[test]
fn test_valid_json_pretty() {
  let mut cmd = Command::cargo_bin("opml").unwrap();
  let assert = cmd.args(&["--file", SAMPLE, "--json-pretty"]).assert();

  assert
    .success()
    .code(0)
    .stdout(read_to_string("tests/snapshots/json-pretty.json").unwrap());
}
