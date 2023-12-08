use std::marker::PhantomData;

use crate::{basic::sigma, string::StringTrait};

use super::{Basic, Run};

///! Print outputs a message to the console
pub struct Print<T, S>(PhantomData<(T, S)>);

impl<Env, T, S> Basic<Env, Print<T, S>>
where
    T: StringTrait,
{
    pub fn step(self) -> Basic<Env, S> {
        println!("{}", T::string());
        sigma()
    }
}

impl<Env, T, S> Run for Basic<Env, Print<T, S>>
where T: StringTrait, Basic<Env, S>: Run
{
    fn run(self) {
        let next = self.step();
        next.run();
    }
}
