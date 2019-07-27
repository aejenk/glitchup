use super::options::MutConfig;

pub trait Mutation {
    fn mutate(&mut self, data : &mut [u8], config : Box<&MutConfig>);
}