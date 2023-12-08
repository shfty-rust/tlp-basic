use std::marker::PhantomData;

use super::{Basic, Run, sigma};

/// Remark passes through to the next instruction without doing anything
pub struct Remark<T, S>(PhantomData<(T, S)>); // Code comment

impl<Env, T, S> Basic<Env, Remark<T, S>> {
    pub const fn step(self) -> Basic<Env, S> {
        sigma()
    }
}

impl<Env, T, S> Run for Basic<Env, Remark<T, S>>
where
    Basic<Env, S>: Run,
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}
