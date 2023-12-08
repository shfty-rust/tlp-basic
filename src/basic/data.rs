use std::marker::PhantomData;

use super::{sigma, Basic, Run, Context, DataContext};

/// Data sets the global list accessed by Read
pub struct Data<V, S>(PhantomData<(V, S)>);

impl<Labels, Variables, D, DataPointer, V, S>
    Basic<Context<Labels, Variables, DataContext<D, DataPointer>>, Data<V, S>>
{
    pub fn step(self) -> Basic<Context<Labels, Variables, DataContext<V, DataPointer>>, S> {
        sigma()
    }
}

impl<Labels, Variables, D, DataPointer, V, S> Run
    for Basic<Context<Labels, Variables, DataContext<D, DataPointer>>, Data<V, S>>
where
    Basic<Context<Labels, Variables, DataContext<V, DataPointer>>, S>: Run,
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        basic::{basic, Data, End},
        number::*,
        List,
    };

    #[test]
    fn data_binding() {
        type DataBindingProgram = Data<List![U1, U2, U4], End>;

        let program = basic::<DataBindingProgram>();
        let _program = program.step();
        //let _program: ! = _program.step();
    }
}
