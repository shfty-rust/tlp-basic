use std::marker::PhantomData;

use crate::{
    boolean::{False, True},
    cons::IndexOp,
};

use super::{sigma, Basic, Context};

/// If executes one of two branches based on a conditional
pub struct If<C, T, E>(PhantomData<(C, T, E)>); // If-then-(else)

impl<Env, T, E> Basic<Env, If<True, T, E>> {
    pub const fn step(self) -> Basic<Env, T> {
        sigma()
    }
}

impl<Env, T, E> Basic<Env, If<False, T, E>> {
    pub const fn step(self) -> Basic<Env, E> {
        sigma()
    }
}

pub struct Variable<K>(PhantomData<K>);

impl<Labels, Variables, Data, V, T, E>
    Basic<Context<Labels, Variables, Data>, If<Variable<V>, T, E>>
where
    Variables: IndexOp<V, Index = True>,
{
    pub fn step_then(self) -> Basic<Context<Labels, Variables, Data>, T> {
        sigma()
    }
}

impl<Labels, Variables, Data, V, T, E>
    Basic<Context<Labels, Variables, Data>, If<Variable<V>, T, E>>
where
    Variables: IndexOp<V, Index = False>,
{
    pub fn step_else(self) -> Basic<Context<Labels, Variables, Data>, E> {
        sigma()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        basic::{basic, End, If, Let, Print, Variable},
        boolean::*,
        number::*,
        String,
    };

    #[test]
    fn if_else_conditional() {
        type TrueConditional = String![
            C o n d i t i o n a l
            Space
            e v a l u a t e d
            Space
            t r u e
            Period
        ];

        type FalseConditional = String![
            C o n d i t i o n a l
            Space
            e v a l u a t e d
            Space
            f a l s e
            Period
        ];

        type TrueProgram = Print<TrueConditional, End>;
        type FalseProgram = Print<FalseConditional, End>;
        type IfElseProgram<T> = If<T, TrueProgram, FalseProgram>;

        // Step each variant of the program until its exit instruction
        let program = basic::<IfElseProgram<True>>();
        let program = program.step();
        let _program = program.step();
        // let _program: ! = program.step();

        let program = basic::<IfElseProgram<False>>();
        let program = program.step();
        let _program = program.step();
        // let _program: ! = program.step();
    }

    #[test]
    fn if_else_variable() {
        type Foo = U0;

        type EvaluatedTrue = String![
            E v a l u a t e d
            Space
            T r u e
        ];

        type EvaluatedFalse = String![
            E v a l u a t e d
            Space
            F a l s e
        ];

        type IfThenElse = If<Variable<Foo>, Print<EvaluatedTrue, End>, Print<EvaluatedFalse, End>>;
        type TrueProgram = Let<Foo, True, IfThenElse>;

        let program = basic::<TrueProgram>();
        let program = program.step();
        let program = program.step_then();
        let _program = program.step();
        //let _program: ! = _program.step();

        type FalseProgram = Let<Foo, False, IfThenElse>;
        let program = basic::<FalseProgram>();
        let program = program.step();
        let program = program.step_else();
        let _program = program.step();
        //let _program: ! = _program.step();
    }
}
