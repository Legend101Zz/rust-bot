// to overcome the issue of assigning unnecessary heap space to nil value in box smart pointer

// use std::ops::Deref;

// #[derive(Debug)]
// enum List {
//     Cons(i32, Option<Box<List>>),
// }

// struct MySmartPointer {
//     value: i32,
// }

// impl MySmartPointer {
//     fn new(x: i32) -> MySmartPointer {
//         MySmartPointer { value: x }
//     }
// }

// impl Deref for MySmartPointer {
//     type Target = i32;
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }

// impl Drop for MySmartPointer {
//     fn drop(&mut self) {
//         println! {"Droppinng the smartPointer object from memory {:?}", self.value}
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use List::Cons;

//     #[test]

//     fn test_custom_smart_pointer() {
//         let list = Cons(1, Some(Box::new(Cons(2, None))));

//         dbg!(list);

//         let a = 50;
//         let b = Box::new(a);

//         println!("{}", 50 == a);
//         println!("{}", 50 == *b); // deref trait
//                                   //println!("{}", b ==a )

//         let sptr = MySmartPointer::new(a);
//         let sptr2 = MySmartPointer::new(*b);
//         println!("{}", *sptr == a);
//     }
// }

use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

struct MySmartPointer<T: std::fmt::Debug> {
    value: T,
}

impl<T: std::fmt::Debug> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer { value: x }
    }
}

impl<T: std::fmt::Debug> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: std::fmt::Debug> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println! {"Droppinng the smartPointer object from memory {:?}", self.value}
    }
}

fn my_fn(str: &str) {
    println!("The string recieved from code is {}", str)
}

#[cfg(test)]
mod test {
    use super::*;
    use List::Cons;

    #[test]

    fn test_custom_smart_pointer_2() {
        let sptr_1 = MySmartPointer::new("Mrigesh Thakur");
        my_fn(&sptr_1);

        let some_vec = MySmartPointer::new(vec![1, 23]);

        for z in &*some_vec {
            println!("The value is {}", z)
        }
    }
}
