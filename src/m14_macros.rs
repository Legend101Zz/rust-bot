macro_rules! our_macro {
    () => {
        1 + 1
    };

    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
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
    use super::*;

    #[test]

    fn test_macro() {
        our_macro!();

        println!("{}", our_macro!(2, 2));

        println! {"Hello{}",2};

        println!("Please enter a floating point number ");

        let some_input = input!(f32);

        println!("Number entered is  : {}", some_input);
    }
}
