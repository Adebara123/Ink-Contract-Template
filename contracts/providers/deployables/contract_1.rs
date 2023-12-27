
use openbrush::traits::Storage;

use crate::providers::data::contract_1::Contract1Storage;

pub trait Contract1Impl:  Storage<Contract1Storage> {

        fn flip(&mut self) {
            
            let mut state = *self.data::<Contract1Storage>();
            state.value = !state.value;
        }

        fn get(&self) -> bool {

            let state = *self.data::<Contract1Storage>();
            state.value
        }
}