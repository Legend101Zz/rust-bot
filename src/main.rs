// mod my_funcs;
// mod other_funcs;

// use crate::my_funcs::add_five;

// use crate::other_funcs::sum_funcs::add_ten;

// use std::collections::{HashMap,BTreeMap};

use array_tool::vec::*;

mod m10_smart_pointers;
mod m11_custom_smart_pointers;
mod m12_linked_list;
mod m1_enums;
mod m2_structs;
mod m3_trait;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_mods;
mod m12_reference_counting_pointer;
mod m13_double_linked_list;
mod math_funcs;

// const OUR_COURSE: &str = "RUST with AutoGPT";

struct Rectangle {
    length: i32,
    width: i32,
}

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
    let mut vec_1 = vec![1, 2, 3];
    let mut some_clousre = || {
        vec_1.push(9);
        println!("Hello from closure {:?}", vec_1);
    };
    some_clousre();
    some_clousre();

    print!("{}", 99_22_800);

    let mut f = min;
    let mut m = max;

    println! {"hello mu {}",f(2,3)}
    println! {"hello mu {}",m(2,3)}

    prints_full_info(prints_name, "Mrigesh", 10);

    let a = vec![0, 1, 2, 3, 4, 5, 6];
    let check = a.iter().any(|&x| x > 0);
    let check2 = a.iter().all(|&x| x > 0);
    let check3 = a.iter().find(|&&x| x > 0);

    print!("{check}{check2}{:?}", check3);

    let rect1: Rectangle = Rectangle {
        length: 5,
        width: 10,
    };

    let area_rect1 = math_funcs::rect_area(&rect1.length, &rect1.width);

    let vect_1 = vec![1, 1, 2, 3, 4, 5, 6];
    let vect_2 = vec![1, 2, 3];

    let intersection = vect_1.intersect(vect_2);

    dbg!(&intersection);
}

fn prints_name(name: &str) {
    println!("the name is{}", name)
}

fn prints_full_info(f: fn(&str), some_one: &str, age: i32) {
    let m = f(some_one);
    println!("{:?} and my age is {}", m, age)
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn min(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}
