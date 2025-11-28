fn main() {
    let hello = || println!("Hello world!");
    let hello = || println!("Bonjour le monde");
    hello();
}
/*
вернет
Bonjour le monde
---
Такое затенение определений замыканий иллюстрирует тот факт, что замыкания подчиняются правилам затенения переменных, а не функций.
Вместо того, чтобы создавать замыкание и немедленно заменять его, нам, вероятно,  потребуется выбрать одно замыкание для выполнения. Можно сделать этот выбор во время компиляции с помощью статической диспетчеризации или во время выполнения с помощью динамической диспетчеризации.
---
Статическая диспетчеризация позволяет программе принимать решения о поведении во время компиляции.
Флаги функций и условная компиляция
Пример:
```Cargo.toml
[package]
name = "hello_bonjour_static"
version = "0.1.0"
edition = "2018"
[features]
english = []
french = []
[dependencies]
```
```main.rs
#[cfg(feature = "english")]
let hello = || println!("Hello World");
#[cfg(feature = "french")]
let hello = || println!("Bonjour le monde");
hello();
```
и можно запускать с флагом
```
cargo run --features english
cargo run --features
```
---
Константные функции
Пример
```main.rs
enum Language { English, French }

const fn hello(language: Language) -> &'static str {
    match language {
        Language::English => "Hello World",
        Language::French => "Bonjour le monde",
    }
}

fn main() {
    println!("{}", hello(Language::English));
}
```
---
Динамическая отправка
Пример:
```
enum Language = { English, French };
let language = Language::English;
let hello = match language {
    Language::English => || println!("Hello World"),
    Language::French => || println!("Bonjour le monde"),
};
```
 */
