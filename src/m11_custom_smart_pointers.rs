// to overcome the issue of assigning unnecessary heap space to nil value in box smart pointer

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod test {
    use super::*;
    use List::{Cons, Nil};

    #[test]

    fn test_custom_smart_pointer() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

        dbg!(list);
    }
}
