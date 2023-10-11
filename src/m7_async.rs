async fn my_async_call(url:&str)->Result<serde_json::Value, reqwest::Error>{

let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;

Ok(response)

}




#[cfg(test)]
mod tests{

use super::*;

#[tokio::test]
async fn test_calls_async_fn(){
let api_url = "https://cat-fact.herokuapp.com/facts/";

let my_res = my_async_call(api_url).await;


match my_res{
  Ok(res) => {dbg!(res)} ,
  Err(_)=>{
    panic!{
      "failed to make api request"
    }
  }
}
}



}