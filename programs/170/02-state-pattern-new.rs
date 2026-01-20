// Как сделать паттерн переходов между состояниями в Rust стиле

// src/main.rs
fn main() {
  let mut post = Post::new();

  post.add_text("Сегодня на обед я ел салат");

  let post = post.request_review();

  let post = post.approve();
   assert_eq!("Сегодня на обед я ел салат", post.content());
}

// src/lib.rs
pub struct Post {
  content: String,
}

pub struct DraftPost {
  content: String
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }
  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: sel.content,
    }
  }
}

pub struct PendingReviewPost {
  content: String,
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}

// Методы request_review и approve берут self во владение, тем самым поглощая экземпляры DraftPost и PendingReviewPost и трансформируя их соответственно в PendingReviewPost и опубликованный пост Post