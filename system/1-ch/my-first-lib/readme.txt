Создание статического библиотечного крейта
---
cargo new --lib my-first-lib
---
cargo build
В target/debug будет libmy_first_lib.rlib
---
cargo run --bin mymain
cargo run --bin mymaintwo
если нет определения [[bin]] в Cargo.toml полезет по умолчанию в файл src/bin