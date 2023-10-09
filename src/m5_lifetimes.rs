// allocate space in memory 
fn test_lifetime(){
let highest_age :&i32;
let new_value : i32;

// initialise vars 
let alice_age :i32 =20;

let alice =Person {
    name:"alice",
    points: &50.2
};

{

    let bob_age:i32 =21 ;
    let check_ag:f32 = 21.2;
    highest_age = largest(&alice_age,&bob_age);

    let alice_check
= largest(&check_ag,alice.points);
    new_value = *highest_age;
}

println!("New Value of age is {}",new_value)
}

fn largest<'a,  T:PartialOrd >(compare_1: &'a T, compare_2:&'a T)->&'a T {

    if compare_1 < compare_2{
        compare_1
    }else{
        compare_2
    }
}

struct Person<'b>{
    name: &'b str,
    points:&'b f32
}