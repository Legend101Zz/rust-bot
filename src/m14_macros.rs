macro_rules! our_macro {
    () => {
        1+1;
    };
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_macro() {
     our_macro!()
    }
}
