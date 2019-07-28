use super::options::MutConfig;

/// A `Mutation` trait to group every kind of `Mutation`.
/// 
/// Must also implement the `Display` trait.
pub trait Mutation : std::fmt::Display {
    fn configure(&mut self, config: Box<&dyn MutConfig>);
    fn mutate(&mut self, data : &mut [u8]);
}