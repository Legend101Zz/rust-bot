fn some_fn(){
    println!("Basic Math functions ")
}


mod maths{

    pub mod basic_math{


        pub fn multiplication(num1: &i32,num2:&i32)->i32{
            let result = num1*num2;
            printing(&result);
            result
        } 
        fn printing(num:&i32){
            println!("the result is {}",num);
            crate::math_funcs::some_fn()
        }
    
    }

    
}

pub fn rect_area(length:&i32,width:&i32)->i32{
use maths::basic_math::multiplication;

multiplication(length,width)
}