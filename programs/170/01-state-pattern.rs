// Паттерн переходов между состояниями — это объектно-ориентированный пат­терн проектирования.

// процесс создания поста в блоге:
// 1. Пост начинается как пустой черновик.
// 2. Когда черновик готов, запрашивается его проверка.
// 3. После того как пост одобрен, он публикуется.
// 4. Печатаются только опубликованные посты, поэтому неодобренные не могут быть опубликованы случайно.

// Код, демонстрирующий желаемое поведение 
// src/main.rs
use blog::Post;

fn main(){
  let mut post = Post::new();

  post.add_text("Сегодня на обед я ел салат");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("Сегодня на обед я ел салат", post.content());
}

// Определение структуры Post и функции new, создающей новый экземпляр структуры Post, а также типаж State и структура Draft
// src/lib.rs
pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }
  pub fn add_text(&mut self, text: &str){
    self.content.push_str(text);
  }
  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(&self)
  }
  pub fn request_review(&mut self) {
    // take, чтобы взять значение Some из поля state и не трогать None
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review)
    }
  }
  pub approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str{
    ""
  }
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview{})
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

struct PendingReview {}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content()
  }
}