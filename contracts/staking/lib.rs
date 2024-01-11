#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
mod staking {
    use openbrush::traits::Storage;
    use staking_app::errors::StakingError;
    use staking_app::traits::staking::*;
    use staking_app::impls::staking::*;

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct StakingContract {
        #[storage_field]
        staking: StakingData,
    }

    impl StakingImpl for StakingContract {}

    impl Staking for StakingContract {
        #[ink(message)]
        fn stake(&mut self, amount: Balance) -> Result<(), StakingError> {
            self.stake_impl(amount)
        }

        #[ink(message)]
        fn unstake(&mut self, amount: Balance) -> Result<(), StakingError> {
            self.unstake_impl(amount)
        }
    }

    impl StakingContract {
        #[ink(constructor)]
        pub fn new(token: AccountId) -> Self {
            let mut contract = Self::default();
            contract.staking.token.set(&token);
            contract
        }
    }  
}
