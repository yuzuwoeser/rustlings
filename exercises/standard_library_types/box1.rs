// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug, Clone)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {

    fn new() -> List {
        List::Nil
    }

    fn from(n: i32, list: List) -> List {
        Cons(n, Box::new(list))
    }

    fn next(&self) -> Option<&List> {
        match self {
            Nil => None,
            Cons(_, n) => Some(&n),
        }
    }

    fn tail(&self) -> Option<&List> {
        let mut tail = self;
        while let Some(next) = tail.next() {
            tail = next
        }
        Some(&tail)
    }

    fn prepend(&mut self, n: i32) {
        *self = List::from(n, self.clone());
    }
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!("List with 1 element: {:?}", List::from(3, Nil));
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::new()
}

pub fn create_non_empty_list() -> List {
    let mut l = List::from(3, Nil);
    l.prepend(2);
    l.prepend(1);
    l.prepend(0);
    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_list() {
        assert_eq!("Cons(3, Nil)", format!("{:?}", List::from(3, Nil)))
    }

    #[test]
    fn test_get_next_or_tail() {
        assert_eq!("Some(Nil)", format!("{:?}", List::from(3, Nil).next()));
        assert_eq!("None", format!("{:?}", List::new().next()));
        assert_eq!("Some(Cons(2, Nil))", format!("{:?}", List::Cons(1, Box::new(List::Cons(2, Box::new(Nil)))).next()));
        assert_eq!("Some(Nil)", format!("{:?}", List::Cons(1, Box::new(List::Cons(2, Box::new(Nil)))).tail()));
    }

    #[test]
    fn test_build_list() {
        let mut l = List::from(2, Nil);
        l.prepend(1);
        assert_eq!("Some(Cons(2, Nil))", format!("{:?}", l.next()));
        assert_eq!("Some(Nil)", format!("{:?}", l.tail()));
        l.prepend(0);
        assert_eq!("Some(Cons(2, Nil))", format!("{:?}", l.next().unwrap().next()));
        assert_eq!("Some(Nil)", format!("{:?}", l.tail()));
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
