use std::cell::RefCell;
use std::rc::Rc;

struct List<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

struct Node<T> {
    element: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: std::fmt::Display> Node<T> {
    fn new(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            element: element,
            prev: None,
            next: None,
        }))
    }
}

impl<T: std::fmt::Display> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, element: T) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
            }

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_dbl_list() {
        let mut list1: List<i32> = List::new();
    }
}
