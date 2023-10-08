// mod my_funcs;
// mod other_funcs;

// use crate::my_funcs::add_five;

// use crate::other_funcs::sum_funcs::add_ten;

// use std::collections::{HashMap,BTreeMap};
mod m1_enums;
mod m2_structs;
mod m3_trait;

// const OUR_COURSE: &str = "RUST with AutoGPT";

fn main() {
    //   println!("Welcome to my nunnu on {}", OUR_COURSE);

    // //stack

    // let x :i32;
    // x =2 ;
    // let y  =4;
    // println!("{}{}",x,y);

    // for i in 0..=y {
    //   if i != 4{
    //     print!{"{},",i}
    //   } else{
    //     print!{"{},",i}
    //   }
    // }

    // let mut z = 5;
    // z =10;

    // let my_floats:[f32;10] = [0.0;10];
    // print!("{:?}",my_floats);

    // let y_floats_new = my_floats.map(|
    // n| n+2.0);
    // print!("{:?}",y_floats_new);

    let add_num = |x: i32| x + 2;

    let new_num = add_num(7);

    dbg!(new_num);

    print!("{}", 99_22_800);
}
