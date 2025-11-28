cargo new --bin first-program && cd first-program
---
Флаг --bin указывает Cargo сгенерировать пакет, который после компиляции создаст двоичный контейнер (исполняемый файл)
first-program — это имя указанного пакета
---
cargo build
---
cargo run
---
Cargo.toml
[[bin]]
name = "new-first-program"
path = "src/main.rs"

cargo run –-bin new-first-program
---
Cargo.toml
[[bin]]
name = "new-first-program"
path = "src/main.rs"
[[bin]]
name = "new-second-program"
path = "src/second.rs"

cargo run --bin new-second-program
---

