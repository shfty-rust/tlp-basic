use std::marker::PhantomData;

use crate::cons::{Set, SetOp};

use super::{sigma, Basic, Context, Run};

/// Let assigns a value to a variable
pub struct Let<K, V, S>(PhantomData<(K, V, S)>); // Do nothing

impl<Labels, Variables, Data, K, V, S> Basic<Context<Labels, Variables, Data>, Let<K, V, S>>
where
    Variables: SetOp<K, V>,
{
    pub fn step(self) -> Basic<Context<Labels, Set<Variables, K, V>, Data>, S> {
        sigma()
    }
}

impl<Labels, Variables, Data, K, V, S> Run for Basic<Context<Labels, Variables, Data>, Let<K, V, S>>
where
    Variables: SetOp<K, V>,
    Basic<Context<Labels, <Variables as SetOp<K, V>>::Set, Data>, S>: Run,
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        basic::{basic, End, Let},
        number::*,
    };

    #[test]
    fn let_binding() {
        type Foo = U0;
        type Bar = U1;
        type Baz = U2;

        type LetBindingProgram = Let<Foo, U1, Let<Bar, U2, Let<Baz, U4, End>>>;

        let program = basic::<LetBindingProgram>();
        let program = program.step();
        let program = program.step();
        let _program = program.step();
        //let _program: ! = _program.step();
    }
}
