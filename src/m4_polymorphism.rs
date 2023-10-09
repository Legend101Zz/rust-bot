  use ethers::types::Address;
  use std::str::FromStr;

trait EthereumAddress{
  fn convert_address(&self)-> Result<Address, &'static str>;
}

impl EthereumAddress for Address {

fn convert_address(&self)-> Result<Address, &'static str> {
    Ok(*self )
}

}

impl EthereumAddress for &str{
  fn convert_address(&self)-> Result<Address, &'static str> {
      match Address::from_str(self){
        Ok(address)=>Ok(
          address
        ),
      Err(_)=>Err("Invalid Ethereum Address ")
      }
  }
}



  fn get_ethereum_data<T:EthereumAddress> (address:T)->Address {
 
    let converted_address:Address = address.convert_address().unwrap();
converted_address
  }


  #[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_poly() {
let addr : Address= Address::from_str("0xb794f5ea0ba39494ce839613fffba74279579268").unwrap();

let new_addr = get_ethereum_data("0xb794f5ea0ba39494ce839613fffba74279579268");
        assert_eq!(new_addr, Address::from_str("0xb794f5ea0ba39494ce839613fffba74279579268").unwrap());
    }



}
