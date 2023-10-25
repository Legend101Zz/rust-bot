// to overcome the issue of assigning unnecessary heap space to nil value in box smart pointer

#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}   

#[cfg(test)]
mod test {
    use super::*;
    use List::Cons;

    #[test]

    fn test_custom_smart_pointer() {
        let list = Cons(1, Some(Box::new(Cons(2, None))));

        dbg!(list);
    }
}
