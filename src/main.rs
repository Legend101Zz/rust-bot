// mod my_funcs;
// mod other_funcs;

// use crate::my_funcs::add_five;

// use crate::other_funcs::sum_funcs::add_ten;

// use std::collections::{HashMap,BTreeMap};
mod m1_enums;
mod m2_structs;
mod m3_trait;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;

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
let mut vec_1 = vec!(1,2,3);
    let mut some_clousre = ||{
        vec_1.push(9);
        println!("Hello from closure {:?}",vec_1);
    };
    some_clousre();
  some_clousre();



    print!("{}", 99_22_800);


    let mut f = min;
let mut m = max;

    println!{"hello mu {}",f(2,3)}
    println!{"hello mu {}",m(2,3)}

    prints_full_info(prints_name,"Mrigesh",10);

}


fn prints_name(name:&str){
    println!("the name is{}",name)
}

fn prints_full_info(f:fn(&str),some_one:&str,age:i32){
    let m= f(some_one);
    println!("{:?} and my age is {}",m,age)
}


fn max(x:i32,y:i32)->i32{
    if x>y {x} else {y}
}

fn min(x:i32,y:i32)->i32{
    if x<y {x} else {y}
}



