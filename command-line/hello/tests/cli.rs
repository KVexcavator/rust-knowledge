/*
use std::process::Command;

#[test]
fn works(){
  assert!(true);
}

#[test]
fn runs(){
  let mut cmd = Command::new("ls");
  let res = cmd.output();
  assert!(res.is_ok());
}


скомпилированная прога hello
находится в target/debug
но н езапустится по умолчанию
надо настроить в Cargo.tomb
[dev-dependencies]
assert_cmd = "1"

#[test]
fn runs_hello(){
  let mut cmd = Command::new("hello");
  let res = cmd.output();
  assert!(res.is_ok());
}
далее надо импортировать
use assert_cmd::Command
и написать чтото вроде
*/
use assert_cmd::Command;
#[test]
fn runs(){
  let mut cmd = Command::cargo_bin("hello").unwrap();
  cmd.assert().success();
}

#[test]
fn true_ok(){
  let mut cmd = Command::cargo_bin("true").unwrap();
  cmd.assert().success();
}

#[test]
fn false_not_ok(){
  let mut cmd = Command::cargo_bin("false").unwrap();
  cmd.assert().failure();
}

#[test]
fn runs_hello_stdout(){
  let mut cmd = Command::cargo_bin("hello").unwrap();
  cmd.assert().success().stdout("Hello, world!\n");
}