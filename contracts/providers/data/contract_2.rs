

#[derive(Debug, Clone, Copy)]
#[openbrush::storage_item()]
pub struct Contract2Storage {
    
  pub value: bool,
}

impl Default for Contract2Storage {
    fn default() -> Self {
        Self {
            value: false
        }
    }
}