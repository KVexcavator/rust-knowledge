// Вариант использования внутренней изменяемости: mock-объекты(обьекты пустышки)
// Тестовый двойник — когда во время тестирования один тип используется вместо другого

// Библиотека, позволяющая отслеживать близость значения к максимальному и предупреждать, когда это значение находится на неких уровнях

pub trait Messenger {
  // берет неизменяемую ссылку на self и текст сообщения
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
  where T: Messenger {
  pub vn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  // мы хотим проверить пове­дение метода set_value в структуре LimitTracker
  pub fn set_value(&mut self, value: usize){
    self.value = value;
    let percentage_of_max = self.value as f64 / self.max as 64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Ошибка: Вы превысили свою квоту!");
    } else if percentage_of_max >= 0.9 {
      self.messenger.send("Срочное предупреждение: Вы израсходовали свыше 90% своей квоты!");
    } else if percentage_of_max >= 0.75 {
      self.messenger.send("Предупреждение: Вы израсходовали свыше 75% своей квоты!");
    }
  }
}

//  Для тестирования нужен mock-объект, который сообщения при вызове метода send будет отслеживать только те сообще­ния, которые должен

// Использование умного указателя RefCell<T> для изменения внутреннего значения, тогда как внешнее значение считается неизменяемым
#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger { sent_messages: RefCell::new(vec![])}
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str){
      self.sent_messages.borrow_mut(),push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message(){
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}
// При создании неизменяемых и изменяемых ссылок мы используем синтаксис & и &mut. 
// С RefCell<Т> мы используем методы за­имствования borrow и borrow_mut. 
// borrow возвращает тип умного указателя Ref<T>
// borrow_mut возвращает тип умного указателя RefMut<Т>.
  