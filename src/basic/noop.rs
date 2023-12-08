use std::marker::PhantomData;

use super::{sigma, Basic, Run};

/// NoOp does nothing
pub struct NoOp<S>(PhantomData<S>); // Do nothing

impl<Env, S> Basic<Env, NoOp<S>> {
    pub const fn step(self) -> Basic<Env, S> {
        sigma()
    }
}

impl<Env, S> Run for Basic<Env, NoOp<S>>
where
    Basic<Env, S>: Run,
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}
