

#[derive(Debug, Clone, Copy)]
#[openbrush::storage_item()]
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