////////////////////////////////////////////////////////////////////////////////////// 
// REALLY simple blueprint to create a new token.                                   //
// This simply creates the token based on a name, symbol, and supply passed to it.  //
// The created bucket with the tokens is returned and put into your account vault.  //
//////////////////////////////////////////////////////////////////////////////////////
use scrypto::prelude::*;

blueprint! {
    struct NewToken;
    impl NewToken {        
        pub fn new(name: String, symbol: String, supply: Decimal) -> Bucket {
            // Create a new token
            ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", name)
                .metadata("symbol", symbol)
                .initial_supply_fungible(supply)
        }
    }
}
