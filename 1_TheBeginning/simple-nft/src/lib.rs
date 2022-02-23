// REALLY simple blueprint to create a new non fungible token. 
// This does nothing but create the token based on a name, symbol, and supply passed to it.
// No mechanism for interacting with the token at all.
use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct GenericNFT {
    name: String,
    description: String,
}

blueprint! {
    struct NewNFT {
        // Vault where the initial nft supply will be
        initial_vault: Vault
    }

    impl NewNFT {        
        pub fn new(name: String, description: String) -> Component {
            // Create a new nft
            let b: Bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "Basic NFT")
                .initial_supply_non_fungible([
                    (
                        NonFungibleKey::from(1u128),
                        GenericNFT {
                            name: name,
                            description: description,
                        },
                    )
                ]);

            Self {
                initial_vault: Vault::with_bucket(b)
            }
            .instantiate()
        }
    }
}
