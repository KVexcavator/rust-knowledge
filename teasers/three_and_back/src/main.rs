use std::f32::consts::PI;

pub struct Degrees(pub f32);
pub struct Radians(pub f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl From<Degrees> for Radians {
    fn from(item: Degrees) -> Self {
        Self(item.0 * PI / 180.0)
    }
}


fn main() {
    let one_eighty_degrees = Degrees::new(180.0);
    let one_eighty_radians: Radians = one_eighty_degrees.into();
    println!("180 Degrees in Radians = {}", one_eighty_radians.0);
}
/*
это вернуло
180 Degrees in Radians = 3.1415927 это норм для Rust
трейт Into не был реализован явно, но когда был определён трейт From, Rust автоматически реализовал обратный трейт Into
В данном примере, если у наc определён From для Degrees, можно спокойно выполнять:
let r : Radians = d.into() or let r = Radians::from(d)
Что бы работало в обратном порядке, надо опрелелить From для Radians
impl From<Radians> for Degrees {
    fn from(item: Radians) -> Self {
        Self(item.0 * (PI / 180.0))
    }
}

Так же можно определить кастомный From, например, ограничить числовой тип, чтобы он принимал только значения от 0 до 10:
use std::convert::TryFrom;

struct ZeroToTen(i32);

impl TryFrom<i32> for ZeroToTen {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value > 10 {
            Err("Value must be between 0 and 10")
        } else {
            Ok(Self(value))
        }
    }
}
здесь автоматически создается обратная TryInto
*/