Установка
https://www.rust-lang.org/tools/install/
rustup - инстру­мент командной строки для управления версиями языка Rust
проверить
rustc --version
rustup --version
установка
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
обновить окружение
source $HOME/.cargo/env
добавить эту строку в ~/.bashrc
=============================
Hello
```hello_world.rs
fn main() {
	println!("Hello, World!");
}
```
компиляция
rustc main.rs
выполнение
./hello_world
---------------------------
Функция main это всегда первый код, который выполняется в каждой исполняемой про­грамме Rust
! означает, что вызывается макрокоманду вместо обычной функции
============================
Cargo — это система сборки и пакетный менеджер языка Rust
https://doc.rust-lang.org/cargo/
устанавливается вместе с rust
cargo --version
создать каталог
cargo new hello_cargo
помощь 
cargo new --help
cargo doc --open выведет документацию, порождаемую всеми зависимостями, локально и откроет ее в браузере
внутри 
кофиг Cargo.toml
файлы с исходным кодом в каталоге src
построить проект
cd hello_cargo
cargo build
создаст а в target/debug/hello_cargo
исполнить
./target/debug/hello_cargo
для компиляции кода и последующего выполнения ре­зультирующего исполняемого файла
cargo run
быстрая проверка кода
cargo check
форматирование кода
cargo fmt
Сборка для релиза
cargo build --release
cargo build
обновить упаковку
cargo update
-----------------------------------
подсказки по коду
cargo clippy
если надо турбо режим для подсказок, вверху main страницы
#![warn(clippy::all, clippy::pedantic)]
-----------------------------------
поиск пакетов по названию и ключевым словам
cargo search bracket-terminal
добавить пакет rand
cargo add rand

добавление пакетов в cargo
• =0.8.0 будет использовать только версию 0.8.0 — ничего ниже или выше.
• ^0.8.0 будет использовать любую версию, равную или выше 0.8.0, но только в диапазоне версий 0.x.
• ~0.8.0 будет использовать любую младшую версию выше 0.8.0. Обновления будут применяться автоматически, даже если они нарушают API контейнера.
другие способы
[dependencies]
bracket-lib = { git = "https://github.com/thebracket/bracket-lib" }
[dependencies]
bracket-lib = {
	git = "https://github.com/thebracket/bracket-lib",
	default-features = false,
	features = [ "amethyst_engine_vulkan" ]
}
===================================
Git repo
$ git clone someurl.com/некийпроект
$ cd некийпроект
$ cargo build