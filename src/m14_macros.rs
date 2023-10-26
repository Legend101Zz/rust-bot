macro_rules! our_macro {
    () => {
        1+1
    };

    ($e1: expr, $e2: expr)=>{
        $e1 + $e2
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_macro() {
     our_macro!();

     println!("{}", our_macro!(2,2))
    }
}
