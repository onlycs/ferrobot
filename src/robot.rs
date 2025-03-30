use crate::ctx::Context;

pub trait Robot: Default {
    fn periodic(&mut self, ctx: &mut Context) {}
}
