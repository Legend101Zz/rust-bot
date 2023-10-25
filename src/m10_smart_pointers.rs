
enum List {
    Cons(i32,Box<List>),
    Nil,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_smart_pointer() {
       
       let single_value = Box::new(0.625);
    let x  = 0.625;
    println!("Are the values equal ? {} ", x == *single_value);
    dbg!{*single_value};

    let stack_var = 4;
    let stack_ref = &stack_var;

 

 use List::{Cons,Nil};

 let list = Cons(1,Box::new(Cons(2,Box::new(Cons::new(3,Box::new(Nil))))));

    let heap_var = Box::new(stack_var);
    
    }

   
}
