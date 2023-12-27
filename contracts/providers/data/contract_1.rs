

#[derive(Debug, Clone, Copy)]
#[openbrush::storage_item(CONTRACT_1_STATE)]
pub struct Contract1Storage {
   
   pub  value: bool,
}

impl Default for Contract1Storage {
    fn default() -> Self {
        Self {
            value: false
        }
    }
}