
use openbrush::traits::Storage;

use crate::providers::data::contract_2::Contract2Storage;

pub trait Contract2Impl:  Storage<Contract2Storage> {

        fn flip(&mut self) {
            
            let mut state = *self.data::<Contract2Storage>();
            state.value = !state.value;
        }

        fn get(&self) -> bool {

            let state = *self.data::<Contract2Storage>();
            state.value
        }
}