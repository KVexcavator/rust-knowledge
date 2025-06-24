use std::process::Command;

#[test]
fn runs() {
  let mut cmd = Command::new("ls");
  let res = cmd.output();
  assert!(res.is_ok());
}

// #[test]
// fn runs_fail() {
//   let mut cmd = Command::new("hello");
//   let res = cmd.output();
//   assert!(res.is_ok());
// }