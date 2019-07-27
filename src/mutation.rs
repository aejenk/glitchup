use super::options::MutConfig;

pub trait Mutation : std::fmt::Display {
    fn mutate(&mut self, data : &mut [u8], config : Box<&MutConfig>);
}