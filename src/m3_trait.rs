trait Attacker {

    fn choose_style(&self)->String ;

}


#[derive(Debug)]
enum Character {
Warrior ,
Archer,
Wizard

}

impl Attacker for Character{
    fn choose_style(&self)->String {
        match self{
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