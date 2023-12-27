#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[ink::contract]
mod contract_2 {


    use global::providers::{data::contract_2::Contract2Storage, deployables::contract_2::Contract2Impl};
    use global::controllers::contract_2::contract2controller_external::Contract2Controller;
   
    #[ink(storage)]
    #[derive(Default, openbrush::traits::Storage)]
    pub struct Contract2 {
        
        #[storage_field]
        pub contract_2_state: Contract2Storage,

    }

    impl  Contract2Impl for Contract2 {}


    impl Contract2Controller for Contract2 {
        #[ink(message)]
        fn flip(&mut self) {
           Contract2Impl::flip(self);
        }

        #[ink(message)]
        fn get(&self) -> bool {
            Contract2Impl::get(self)
        }
    }

    impl Contract2 {
      
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default() 
        }
      
      
    }

}