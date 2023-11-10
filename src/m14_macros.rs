macro_rules! mad_skills {
// ($x:expr)=>{
//     format!("You sent an expression :{}",$x)
// }
($x :ty)=>{
    match stringify!($x){
        "i32" => "You sent an i32 type".to_string(),
        _=>"You sent something else".to_string()
    }
}
}


macro_rules! string_concat {
 

    ($($some_str:expr),*)=>{{
        let mut temp_str = String::new();

    $(temp_str.push_str($some_str);)*;
    temp_str}
    }
}

macro_rules! input {
    ($t :ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("failed to read input");

        let n: $t = n.trim().parse().expect("invalid input");

        n}
    };
}

macro_rules! my_vec {
    ($($x:expr),+) => {
        {
            let mut temp_vec = Vec::new();

        $(
            temp_vec.push($x);
         )+
         temp_vec

       
        }
    };
}
 
#[cfg(test)]
mod test {


    #[test]

    fn test_macro() {
        // our_macro!();

        // println!("{}", our_macro!(2, 2));

        // println! {"Hello{}",2};

        // println!("Please enter a floating point number ");

        // let some_input = input!(f32);

        // println!("Number entered is  : {}", some_input);

        let str_null = string_concat!();
        let str_single = string_concat!("First");

        let str_multiple = string_concat!("Firts","Second");

        let some_var = mad_skills!(i32);
        let mut x: Vec<i32>  = vec!();
        let mut y = my_vec!(1,2,3,4);

        dbg!(y);
    }
}
