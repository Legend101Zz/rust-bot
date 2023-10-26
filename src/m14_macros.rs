macro_rules! our_macro {
    () => {
        1 + 1
    };

    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
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

#[cfg(test)]
mod test {
    use std::ffi::c_long;

    use super::*;

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

        println!("{}",str_multiple)
    }
}
