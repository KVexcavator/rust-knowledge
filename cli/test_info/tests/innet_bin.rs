use assert_cmd::Command;

#[test]
fn runs_inner_bin() {
  let mut cmd = Command::cargo_bin("test_info").unwrap();
  cmd.assert().success();
}

#[test]
fn true_ok(){
  let mut cmd = Command::cargo_bin("true").unwrap();
  cmd.assert().success();
}

#[test]
fn output_eq() {
  let mut cmd = Command::cargo_bin("test_info").unwrap();
  cmd.assert().success().stdout("Hello, world!\n");
}