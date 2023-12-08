use std::marker::PhantomData;

use crate::basic::sigma;

use super::{Basic, Run};

///! Clears the console and positions the cursor at the top-left
pub struct ClearScreen<S>(PhantomData<S>);

impl<Env, S> Basic<Env, ClearScreen<S>>
{
    pub fn step(self) -> Basic<Env, S> {
        print!("\x1B[2J\x1B[1;1H");
        sigma()
    }
}

impl<Env, S> Run for Basic<Env, ClearScreen<S>>
where
    Basic<Env, S>: Run,
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}
