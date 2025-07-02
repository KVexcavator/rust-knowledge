// Определение типажа Summary с реализацией по умолчанию метода summary
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Читать дальше...)")
  }
}
// Чтобы воспользоваться реализацией по умолчанию для суммирования экземпля­ ров типа NewsArticle, вместо определения настраиваемой реализации мы указы­ваем пустой блок
impl Summary for NewsArticle {}

// Реализации по умолчанию могут вызывать другие методы с тем же типажом, даже если эти методы не имеют реализации по умолчанию.
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Читать дальше в {}...)", self.summarize_author())
  }
}
// Чтобы воспользоваться этой версией Summary, нам нужно определить функцию summarize_author, только когда мы реализуем типаж в типе:
impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}