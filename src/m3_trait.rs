trait Attacker {
    fn choose_style(&self) -> String;
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

#[derive(Debug)]

struct Data {
    some_data: Vec<i32>,
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum = 0;
        for i in self.some_data.iter() {
            sum += *i;
        }
        sum as f32 / self.some_data.len() as f32
    }

    fn variance(&self) -> f32 {
        let mu = self.mean();
        let mut sum_square_diff: f32 = 0.0;

        for i in self.some_data.iter() {
            sum_square_diff += (*i as f32 - mu) * (*i as f32 - mu);
        }
        sum_square_diff / self.some_data.len() as f32
    }
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "wing chung".to_string(),
            Character::Archer => "KingFU".to_string(),
            Character::Wizard => "Wizard".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_traits() {
        let my_character = Character::Archer;

        let chosen_fighting_style = my_character.choose_style();

        dbg!(chosen_fighting_style);
    }
}
