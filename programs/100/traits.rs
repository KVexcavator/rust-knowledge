// Типажи
// Реализация типажа Summary в типах NewsArticle и Tweet
pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("конечно, как вы, наверное, уже знаете, люди"),
    reply: false,
    retweet: false,
  }
  println!("1 новый твит: {}", tweet.summarize());
}

// Типажи в качестве параметров
// Этот параметр принимает любой тип, реализующий указанный ти­паж. 
// Можно передать экземпляр любого из двух типов, NewsArticle или Tweet
pub fn notify(item: impl Summary) {
  println!("Срочные новости! {}", item.summarize());
}

// Синтаксис границы типажа
// impl Trait является синтаксическим сахаром для более длинной формы
pub fn notify<T: Summary>(item: T) {}
// это может быть удобно, например
pub fn notify(item1: impl Summary, item2: impl Summary){}
pub fn notify<T: Summary>(item1: T, item2: T) {}

// Указание нескольких границ типажа с помощью синтаксиса +
// item должен реализовать как Display, так и Summary
pub fn notify(item: impl Summary + Display){}
pub fn notify<T: Summary + Display>(item: T){}

// Более четкие границы типажа с условием where
// чтобы не писать
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// можно
fn some_function<T, U>(t: T, u: U) -> i32
  where T: Display + Clone,
    U: Clone + Debug
{}

// Возвращение типов, реализующих типажи
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("конечно, как вы, наверное, уже знаете, люди"),
    reply: false,
    retweet: false,
  }
}
// !!можно использовать impl Trait только в том случае, если возвращается один тип, из за ограничений компилятора