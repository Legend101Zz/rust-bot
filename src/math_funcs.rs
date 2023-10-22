mod maths{

    pub mod basic_math{
        use super::printing;

        pub fn multiplication(num1: &i32,num2:&i32)->i32{
            let result = num1*num2;
            printing(&result);
            result
        } 
    }

    pub fn printing(num:&i32){
        println!("the result is {}",num)
    }

}

pub fn rect_area(length:&i32,width:&i32)->i32{
use maths::basic_math::multiplication;

multiplication(length,width)
}