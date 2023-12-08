use std::marker::PhantomData;

use crate::number::U0;

use super::{sigma, Basic, Context, DataContext, Run};

/// Restore resets the internal data pointer to zero
pub struct Restore<S>(PhantomData<S>);

impl<Labels, Variables, Data, DataPointer, S>
    Basic<Context<Labels, Variables, DataContext<Data, DataPointer>>, Restore<S>>
{
    pub const fn step(self) -> Basic<Context<Labels, Variables, DataContext<Data, U0>>, S> {
        sigma()
    }
}

impl<Labels, Variables, Data, DataPointer, S> Run
    for Basic<Context<Labels, Variables, DataContext<Data, DataPointer>>, Restore<S>>
where
    Basic<Context<Labels, Variables, DataContext<Data, U0>>, S>: Run,
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        basic::{basic, Data, End, Read},
        number::*,
        List,
    };

    #[test]
    fn data_restore() {
        struct Foo;
        struct Bar;
        struct Baz;
        type DataRestoreProgram =
            Data<List![U1, U2, U4], Read<Foo, Restore<Read<Bar, Read<Baz, End>>>>>;

        let program = basic::<DataRestoreProgram>();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let _program = program.step();
        //let _program: ! = program.step();
    }
}
