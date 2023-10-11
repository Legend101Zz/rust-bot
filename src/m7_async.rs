use::std::io::{Error,ErrorKind};


async fn my_async_call(url:&str)->Result<serde_json::Value,Error>{

let response = reqwest::get(url).await.map_err(|_|Error::new(ErrorKind::Other,"Could not retrieve response") )?;


 let json_response = response.json::<serde_json::Value>().await.map_err(|_|Error::new(ErrorKind::Other,"Could not decode to json") )?;

  
Ok(json_response)

}




#[cfg(test)]
mod tests{

use ethers::types::Res;

use super::*;

#[tokio::test]
async fn test_calls_async_fn(){
let api_url = "https://cat-fact.herokuapp.com/facts/";

let my_res: Result<serde_json::Value,std::io::Error> = my_async_call(api_url).await;


match my_res{
  Ok(r) => {dbg!(r);} ,
  Err(_)=>{
    panic!{
      "failed to make api request"
    };
  }
}
}



}