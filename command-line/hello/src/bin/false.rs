fn main(){
  //std::process::exit(1);
  // тоже
  std::process::abort();
}
/*
$ cargo run --quiet --bin false
$ echo $?
1
 */