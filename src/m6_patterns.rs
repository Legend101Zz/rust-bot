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
