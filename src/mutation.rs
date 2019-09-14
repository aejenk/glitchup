use super::options::MutConfig;

/// A `Mutation` trait to group every kind of `Mutation`.
/// 
/// Must also implement the `Display` trait.
pub trait Mutation : std::fmt::Display + std::fmt::Debug + MutationClone {
    fn configure(&mut self, config: Box<&dyn MutConfig>);
    fn mutate(&mut self, data : &mut [u8]);
}

pub trait MutationClone {
    fn clone_box(&self) -> Box<Mutation>;
}

impl<T> MutationClone for T where
    T: 'static + Mutation + Clone {
        fn clone_box(&self) -> Box<Mutation> {
            Box::new(self.clone())
        }
}

impl Clone for Box<Mutation> {
    fn clone(&self) -> Box<Mutation> {
        self.clone_box()
    }
}