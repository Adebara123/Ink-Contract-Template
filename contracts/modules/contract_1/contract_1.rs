#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[ink::contract]
pub mod contract_1 {


    use global::providers::{data::contract_1::Contract1Storage, deployables::contract_1::Contract1Impl};
    use global::controllers::contract_1::contract1controller_external::Contract1Controller;
   
    #[ink(storage)]
    #[derive(Default, openbrush::traits::Storage)]
    pub struct Contract1 {
        #[storage_field]
        pub contract_1_state: Contract1Storage,

    }

    impl  Contract1Impl for Contract1 {}


    impl Contract1Controller for Contract1 {
        #[ink(message)]
        fn flip(&mut self) {
           Contract1Impl::flip(self);
        }

        #[ink(message)]
        fn get(&self) -> bool {
            Contract1Impl::get(self)
        }
    }

    impl Contract1 {
      
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default() 
        }
      
      
    }

}