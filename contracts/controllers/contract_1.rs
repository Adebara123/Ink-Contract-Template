

#[openbrush::wrapper]
pub type APoolRef = dyn Contract1Controller ;

#[openbrush::trait_definition]
pub trait Contract1Controller {
    
    #[ink(message)]
    fn flip(&mut self);

    #[ink(message)]
    fn get(&self) -> bool;

}