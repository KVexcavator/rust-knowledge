use std::mem::size_of;

struct VeryImportantMessage {
    _message_type: u8,
    _destination: u16
}
fn main() {
    println!(
        "VeryImportantMessage occupies {} bytes.",
        size_of::<VeryImportantMessage>()
    );
}
/*
вернуло
VeryImportantMessage occupies 4 bytes.

Размеры message_type и _destination занимают 1 и 2 байта памяти соответственно. 
Так почему же VeryImportantMessage занимает 4 байта памяти?

24-битная (3-байтовая) структура естественным образом не выравнивается по 32-битной карте памяти, поэтому по умолчанию Rust тратит 8 бит памяти на структуру, чтобы обеспечить быстрый доступ к структуре в памяти компьютера.

Ограничение оптимизатора Rust
Можно отключить обе оптимизации структуры Rust с помощью декоратора #[repr()], который позволяет контролировать представление структуры в памяти:
• #[repr(C)] объявляет о необходимости взаимодействия с языком C. Rust не будет переупорядочивать содержимое вашей структуры.
• #[repr(packed)] сообщает Rust, что не следует тратить место на вашу структуру. Это может
привести к небольшому снижению производительности, но гарантирует, что структуры будут иметь правильный размер.
Можно комбинировать:
#[repr(C, packed)]
struct ReallyThreeBytes {
    a: u8,
    b: u16
}
fn main() {
    println!(
        "ReallyThreeBytes occupies {} bytes.",
        size_of::<ReallyThreeBytes>()
    );
}
=>
ReallyThreeBytes occupies 3 bytes.
 */