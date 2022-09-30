use {
  assert_cmd::Command, insta::assert_display_snapshot, test_case::test_case,
};

const SAMPLE: &str = "tests/sample.opml";

#[test_case(&["--file", SAMPLE, "--json"], "json" ; "json")]
#[test_case(&["--file", SAMPLE, "--json-pretty"], "json_pretty" ; "json_pretty")]
#[test_case(&["--file", SAMPLE, "--rss"], "rss" ; "rss")]
fn test_valid(args: &[&str], name: &str) {
  let mut cmd = Command::cargo_bin("opml").unwrap();
  let assert = cmd.args(args).assert().success().code(0);
  let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
  assert_display_snapshot!(name, output);
}

#[test_case(&["--rss"], "missing_file" ; "missing_file")]
#[test_case(&["--file", SAMPLE], "missing_format" ; "missing_format")]
#[test_case(&["--rss", "--json"], "multiple_formats" ; "multiple_formats")]
fn test_invalid(args: &[&'static str], name: &str) {
  let mut cmd = Command::cargo_bin("opml").unwrap();
  let assert = cmd.args(args).assert().failure().code(2);
  let output = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
  assert_display_snapshot!(name, output);
}
