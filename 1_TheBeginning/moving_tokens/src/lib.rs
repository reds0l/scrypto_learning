use scrypto::prelude::*;

blueprint! {
    struct MovingTokens {
        // Vault where the initial token supply will be
        initial_vault: Vault,
        // xrd Vault
        xrd_vault: Vault,
    }

    impl MovingTokens {        
        pub fn new(name: String, symbol: String, supply: Decimal) -> Component {
            // Create a new token
            let b: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", name)
                .metadata("symbol", symbol)
                .initial_supply_fungible(supply);

            Self {
                initial_vault: Vault::with_bucket(b),
                xrd_vault: Vault::new(RADIX_TOKEN),
            }
            .instantiate()
        }
        pub fn get_amount(&self) -> Decimal {
            self.initial_vault.amount()
        }

        pub fn take_tokens_by_amount(&mut self, amount: Decimal) -> Bucket {
            self.initial_vault.take(amount)
        }

        pub fn take_all_tokens(&mut self) -> Bucket {
            self.initial_vault.take_all()
        }

        pub fn put_tokens(&mut self, bucket: Bucket) {
            self.initial_vault.put(bucket)
        }

        pub fn trade_xrd_for_tokens_one_to_one(&mut self, bucket: Bucket) -> Bucket {
            let amount = bucket.amount();
            // put xrd into vault
            self.xrd_vault.put(bucket);
            // take created token and send to user
            self.initial_vault.take(amount)
        }
    }
}
