use std::marker::PhantomData;

use crate::cons::{Index, IndexOp};

use super::{sigma, Basic, Run, Context};

/// Goto jumps to the provided label
pub struct Goto<N>(PhantomData<N>);

impl<Labels, Variables, Data, N> Basic<Context<Labels, Variables, Data>, Goto<N>>
where
    Labels: IndexOp<N>,
{
    pub fn step(self) -> Basic<Context<Labels, Variables, Data>, Index<Labels, N>> {
        sigma()
    }
}

impl<Labels, Variables, Data, N> Run for Basic<Context<Labels, Variables, Data>, Goto<N>>
where
    Labels: IndexOp<N>,
    Basic<Context<Labels, Variables, Data>, Index<Labels, N>>: Run,
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}
