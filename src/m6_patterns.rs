#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_match_literals() {
let number : i32 =20;

match number{
1=> println!("1"),
2 |3|4|5|7|15|20 => println!("Got it"),
_=>println!("no no")

}
    }
}
