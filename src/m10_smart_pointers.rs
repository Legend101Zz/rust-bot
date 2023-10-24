
#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_smart_pointer() {
       
       let single_value = Box::new(0.625);
    let x  = 0.625;
    println!("Are the values equal ? {} ", x == *single_value);
    dbg!{*single_value};
    
    }

   
}
