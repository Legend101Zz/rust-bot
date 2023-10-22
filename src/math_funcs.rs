mod maths{

    pub mod basic_math{
        pub fn multiplication(num1: &i32,num2:&i32)->i32{
            let result = num1*num2;
            result
        } 
    }

}

fn rect_area(length:&i32,width:&i32)->i32{
use maths::basic_math::multiplication;

multiplication(length,width)
}