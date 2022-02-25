////////////////////////////////////////////////////////////////////////////////////// 
// REALLY simple blueprint to create a new nft.                                     //
// This simply creates the nft based on a name, symbol, and supply passed to it.    //
// The created bucket with the nft is returned and put into your account vault.     //
//////////////////////////////////////////////////////////////////////////////////////
use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct GenericNFT {
    name: String,
    description: String,
}

blueprint! {
    struct NewNFT;
    impl NewNFT {        
        pub fn new(name: String, description: String) -> Bucket {
            // Create a new nft
            ResourceBuilder::new_non_fungible()
                .initial_supply_non_fungible([
                    (
                        NonFungibleKey::from(1u128),
                        GenericNFT {
                            name: name,
                            description: description,
                        },
                    )
                ])
        }
    }
}
