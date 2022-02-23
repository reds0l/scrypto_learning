// REALLY simple blueprint to create a new token. 
// This does nothing but create the token based on a name, symbol, and supply passed to it.
// No mechanism for interacting with the token at all.
use scrypto::prelude::*;

blueprint! {
    struct NewToken {
        // Vault where the initial token supply will be
        initial_vault: Vault
    }

    impl NewToken {        
        pub fn new(name: String, symbol: String, supply: Decimal) -> Component {
            // Create a new token
            let b: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", name)
                .metadata("symbol", symbol)
                .initial_supply_fungible(supply);

            Self {
                initial_vault: Vault::with_bucket(b)
            }
            .instantiate()
        }
    }
}
