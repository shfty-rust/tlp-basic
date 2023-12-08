use std::marker::PhantomData;

use crate::cons::{Set, SetOp};

use super::{sigma, Basic, Run, Context};

/// Label pushes the current program type into the environment type
pub struct Label<T, S>(PhantomData<(T, S)>);

impl<Labels, Variables, Data, T, S> Basic<Context<Labels, Variables, Data>, Label<T, S>>
where
    Labels: SetOp<T, S>,
{
    pub fn step(self) -> Basic<Context<Set<Labels, T, S>, Variables, Data>, S> {
        sigma()
    }
}

impl<Labels, Variables, Data, T, S> Run
    for Basic<Context<Labels, Variables, Data>, Label<T, S>>
where
    Labels: SetOp<T, S>,
    Basic<Context<<Labels as SetOp<T, S>>::Set, Variables, Data>, S>: Run,
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}
