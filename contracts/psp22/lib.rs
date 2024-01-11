#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
mod staking_contract {
    // use openbrush::contracts::psp22::*;
    use openbrush::test_utils::*;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct StakingContract {
        #[storage_field]
        psp22: psp22::Data
    }

    #[overrider(psp22::Internal)]
    fn _before_token_transfer(&mut self, from: Option<&AccountId>, to: Option<&AccountId>, amount: &Balance) -> Result<(), PSP22Error> {
        if from == Some(&[0; 32].into()) {
            return Err(PSP22Error::Custom(String::from("Transfer from zero address not allowed")));
        }
        Ok(())
    }

    impl StakingContract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut contract = Self::default();
            psp22::Internal::_mint_to(&mut contract, Self::env().caller(), total_supply).expect("Could not mint");
            contract
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let accounts = accounts();
            let mint_amount = 10_000_000;

            let staking_contract = StakingContract::new(mint_amount);

            let alice_balance = PSP22::balance_of(&staking_contract, accounts.alice);

            assert_eq!(alice_balance, mint_amount);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = accounts();
            let mint_amount = 10_000_000;
            let transfer_amount = 1_000;

            let mut staking_contract = StakingContract::new(mint_amount);
            let result = PSP22::transfer(&mut staking_contract, accounts.bob, transfer_amount, Vec::<u8>::new());

            let alice_balance = PSP22::balance_of(&staking_contract, accounts.alice);
            let bob_balance = PSP22::balance_of(&staking_contract, accounts.bob);

            assert!(result.is_ok());
            assert_eq!(alice_balance, mint_amount - transfer_amount);
            assert_eq!(bob_balance, transfer_amount);
        }
    }
}
