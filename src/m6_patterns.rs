#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            print!("I Quit !!")
        }
        Message::ChangeColor(red, green, blue) => {
            println!("Red{} , Green{}, Blue {}", red, green, blue)
        }
        Message::Move { x, y } => {
            println!("X is {} , Y {}", x, y)
        }
        Message::Write(text) => {
            print!("{}", text)
        }
    }
}

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
