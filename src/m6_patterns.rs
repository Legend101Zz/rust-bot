#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_match_literals() {
        let number: i32 = 20;

        match number {
            1 => println!("1"),
            2 | 3 | 4 | 5 | 7 | 15 | 20 => println!("Got it"),
            _ => println!("no no"),
        }
    }

    #[test]

    fn test_match_option() {
        let some_num = Some(10);
        let prob_none: Option<i32> = None;

        let res = match some_num {
            Some(i) => i,
            None => {
                panic!("there was some problem");
            }
        };

        if let Some(i) = some_num {
            dbg!(i);
        }

        print!("Hellooo {}", res)
    }
}
