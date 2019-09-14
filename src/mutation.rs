use super::options::MutConfig;

type Mut = Mutation + Send + Sync;

/// A `Mutation` trait to group every kind of `Mutation`.
/// 
/// Must also implement the `Display` trait.
pub trait Mutation : std::fmt::Display + std::fmt::Debug + MutationClone{
    fn configure(&mut self, config: Box<&dyn MutConfig>);
    fn mutate(&mut self, data : &mut [u8]);
}

pub trait MutationClone {
    fn clone_box(&self) -> Box<Mut>;
}

impl<T> MutationClone for T where
    T: 'static + Mutation + Clone + Send + Sync {
        fn clone_box(&self) -> Box<Mut> {
            Box::new(self.clone())
        }
}

impl Clone for Box<Mut> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}