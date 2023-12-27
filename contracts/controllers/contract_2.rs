

#[openbrush::wrapper]
pub type APoolRef = dyn Contract2Controller ;

#[openbrush::trait_definition]
pub trait Contract2Controller {
    
    #[ink(message)]
    fn flip(&mut self);
    
    #[ink(message)]
    fn get(&self) -> bool;
 
}