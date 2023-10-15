#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]

enum GivenOptions<T> {
    None,
    Some(T),
}


#[derive(Debug)]

enum Conveyance{
     Car = 15,
     Train =20,
     Air =30
}

impl Conveyance {

    fn travel_allowance(&self,miles:i32)->f32{

        let allowance = match self{
            Conveyance::Car => miles as f32 *14.0*2.0,
            Conveyance::Train => miles as f32 *18.0*2.0,
            Conveyance::Air => miles as f32 * 30.0 *2.0
        };

allowance
    }
}



fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under".to_string())
    }
}

fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not under".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOptions<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOptions::Some(remainder)
    } else {
        GivenOptions::None
    }
}

fn handle_car_colour_red() -> CarColour {
    let my_car_color = CarColour::Red;

    my_car_color
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_enums() {
        let car_color: CarColour = handle_car_colour_red();

        dbg!(car_color);

        let is_under_five_res = check_under_five(3);

        dbg!(&is_under_five_res);

        let remainder = remainder_zero(10.0);

        dbg!(remainder);

        let is_under_five_res_buil_in = check_under_five_built_in(6);

        dbg!(is_under_five_res_buil_in);


        let participant_1 = Conveyance::Car;
        dbg!(participant_1.travel_allowance(60));
    }
}
