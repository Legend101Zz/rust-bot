// allocate space in memory 
fn test_lifetime(){
let highest_age :&i32;
let new_value : i32;

// initialise vars 
let alice_age :i32 =20;

{

    let bob_age:i32 =21 ;
    
    highest_age = largest(&alice_age,&bob_age);
    new_value = *highest_age;
}

println!("New Value of age is {}",new_value)
}

fn largest<'a>(compare_1: &'a i32, compare_2:&'a i32)->&'a i32{

    if compare_1 < compare_2{
        compare_1
    }else{
        compare_2
    }
}