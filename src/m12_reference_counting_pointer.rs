use std::rc::Rc;


enum List{
    Cons (i32, Rc<List>),
    Nil
}


use List::{Cons, Nil};

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_rc_pointer() {
      let a = Rc::new(Cons(1,Rc::new(Cons(2,Rc::new(Nil)))));

      println!("Count after creating a ={}", Rc::strong_count(&a));
let b = Rc::new(Cons(3,Rc::clone(&a)));
println!("Count after creating b ={}", Rc::strong_count(&a));
let c = Rc::new(Cons(4,Rc::clone(&a)));
println!("Count after creating c ={}", Rc::strong_count(&a));


    }
}







