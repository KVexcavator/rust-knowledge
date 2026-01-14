// Создание древовидной структуры данных:
// узел с дочерними узлами

use std::rc::Rc;
use std::cell::RefCell;

// структура Node владеет своими дочерними структурами Node и делиться этим владением с переменными, чтобы можно было обращаться к каждому узлу Node в дереве напрямую. 
//Для этого определяем элементы типа Vec<T> как значения типа Rc<Node>. 
// для изменения того, какие узлы являются дочерними другому узлу, в поле children свойство Vec<Rc<Node>> оборачивается в умный указатель RefCell<T>.
#[derive(Debug)]
struct Node {
  value: i32,
  children: RefCell<Vec<Rc<Node>>>,
}

// Создание узла leaf без дочерних узлов и узла branch с leaf в качестве одного из его дочерних узлов

fn main(){
  let leaf = Rc::new(Node{
    value: 3,
    children: RefCell::new(vec![]),
  });

  let branch = Rc::new(Node{
    value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  }),
}
// cейчас у leaf два владельца, и leaf не знает о branch

// Добавление ссылки из дочернего узла на родительский
#[derive(Debug)]
struct Node {
  value: i32,
  // узел сможет ссылаться на родительский узел, но не будет владеть своим родителем
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>,  
}

fn main(){
  let leaf = Rc::new(Node{
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!("родительский узел leaf = {:?}", leaf.parent.borrow().upgrade());

  let branch = Rc::new(Node{
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  // в узле branch появился экземпляр структуры Node, и можно модифицировать узел leaf, чтобы дать ему ссылку Weak<Node> на его родителя 
  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

  println!("родительский узел leaf = {:?}", leaf.parent.borrow().upgrade());
}
