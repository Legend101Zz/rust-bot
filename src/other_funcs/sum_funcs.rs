/// Function: add_ten
/// 
/// # Arguments {num: u32}
/// 
/// #Returns u32
/// 
/// #Example 
/// ```
/// let x =5
/// let y = add_five(x);
/// ```


pub fn add_ten(num: u32) -> u32 {
    num + 10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn add_ten_test() {
        let x: u32 = 100;
        let y: u32 = add_ten(x);
        println! {"y is from test: {}",y}
        assert_eq!(y, 110);
    }
}
