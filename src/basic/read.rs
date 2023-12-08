use std::marker::PhantomData;

use crate::{
    cons::{Index, IndexOp, Insert, InsertOp},
    number::{Add, AddOp, U0, U1},
};

use super::{sigma, Basic, Context, DataContext, Run};

/// Read reads a value from a Data statement, assigns to to a variable,
/// and increments the internal data pointer
pub struct Read<K, S>(PhantomData<(K, S)>);

impl<Labels, Variables, Data, DataPointer, K, S>
    Basic<Context<Labels, Variables, DataContext<Data, DataPointer>>, Read<K, S>>
where
    Variables: InsertOp<U0, (K, Index<Data, DataPointer>)>,
    Data: IndexOp<DataPointer>,
    DataPointer: AddOp<U1>,
{
    pub fn step(
        self,
    ) -> Basic<
        Context<
            Labels,
            Insert<Variables, U0, (K, Index<Data, DataPointer>)>,
            DataContext<Data, Add<DataPointer, U1>>,
        >,
        S,
    > {
        sigma()
    }
}

impl<Labels, Variables, Data, DataPointer, K, S> Run
    for Basic<Context<Labels, Variables, DataContext<Data, DataPointer>>, Read<K, S>>
where
    Variables: InsertOp<U0, (K, Index<Data, DataPointer>)>,
    Data: IndexOp<DataPointer>,
    DataPointer: AddOp<U1>,
    Basic<
        Context<
            Labels,
            <Variables as InsertOp<U0, (K, <Data as IndexOp<DataPointer>>::Index)>>::Insert,
            DataContext<Data, <DataPointer as AddOp<U1>>::Add>,
        >,
        S,
    >: Run,
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
        basic::{basic, Data, End},
        number::*,
        List,
    };

    #[test]
    fn data_read() {
        struct Foo;
        struct Bar;
        struct Baz;
        type DataReadProgram = Data<List![U1, U2, U4], Read<Foo, Read<Bar, Read<Baz, End>>>>;

        let program = basic::<DataReadProgram>();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let _program = program.step();
        //let _program: ! = program.step();
    }
}
