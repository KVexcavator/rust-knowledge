fn main() {
    println!("Hello, world!");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Hello {}!", name.trim());
}
