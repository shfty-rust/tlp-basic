// TODO
//
// [✓] READ instruction
//     [✓] Add ReadIndex generic to Basic
//     [✓] Create Read type
//         * Assign data value to variable
//         * Increment ReadIndex
//
// [✓] Key parameter for Label
//     [✗] Lookup label by key in Goto
//     [✓] Use cons list as an int -> program type map
//
// [✓] Variable evaluation
//     * Will this need a wrapper type for access, ex. Variable<Key>?
//     * Already have math expressions etc, so an Expression type doesn't seem correct
//
// [>] Trait for running a Basic program to completion
//     * Implement over each Basic<...> instruction
//     * Invoke recursively by calling the step() function of the next instruction
//     * Note: Will throw a compile error for infinite loops
//             Can this be made lazy, like an iterator?
//             Ex. program.take(50).run() or program.run(50)?
//
mod cls;
mod data;
mod end;
mod goto;
mod r#if;
mod label;
mod r#let;
mod noop;
mod print;
mod read;
mod remark;
mod restore;

pub use cls::*;
pub use data::*;
pub use end::*;
pub use goto::*;
pub use label::*;
pub use noop::*;
pub use print::*;
pub use r#if::*;
pub use r#let::*;
pub use read::*;
pub use remark::*;
pub use restore::*;

use std::marker::PhantomData;

use crate::number::U0;

/// Type-level BASIC grammar
pub struct Basic<Env, S>(PhantomData<(Env, S)>);

/// Type-level BASIC state
pub struct Context<Labels, Variables, Data>(PhantomData<(Labels, Variables, Data)>);

/// Type-level data list and pointer
pub struct DataContext<Data, DataPointer>(PhantomData<(Data, DataPointer)>);

/// Convenience function to construct an instance of Basic from some program type
pub const fn basic<S>() -> Basic<Context<(), (), DataContext<(), U0>>, S> {
    sigma()
}

/// Convenience function for constructing a type-inferred instance of Basic
/// Used to safely transmute Basic from one sub-type to another
const fn sigma<Env, S>() -> Basic<Env, S> {
    Basic(PhantomData)
}

/// Trait for running a basic program to completion
pub trait Run {
    fn run(self);
}

#[macro_export]
macro_rules ! Basic {
    ($ident:ident: $($tt:tt)*) => {
        $crate::basic::Label<$ident, $crate::Basic!($($tt)*)>
    };

    (REM $ident:ident $($tt:tt)*) => {
        $crate::basic::Remark<$ident, $crate::Basic!{$($tt)*}>
    };

    (CLS $($tt:tt)*) => {
        $crate::basic::ClearScreen<$crate::Basic!{$($tt)*}>
    };

    (PRINT $ident:ident $($tt:tt)*) => {
        $crate::basic::Print<$ident, $crate::Basic!{$($tt)*}>
    };

    (GOTO $ident:ty) => {
        $crate::basic::Goto<$ident>
    };

    (LET $key:ty = $value:ident $($tt:tt)*) => {
        $crate::basic::Let<$key, $value, $crate::Basic!{$($tt)*}>
    };

    (END $($tt:tt)*) => {
        $crate::basic::End
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{number::*, String};

    type HelloWorld = String![
        H e l l o
        Comma
        Space
        W o r l d
        Exclamation
    ];

    #[test]
    fn basic_hello_world() {
        // Simple hello world
        struct ThisIsABasicHelloWorldProgram;
        type HelloWorldProgram =
            Remark<ThisIsABasicHelloWorldProgram, ClearScreen<Print<HelloWorld, End>>>;

        let program = basic::<HelloWorldProgram>();
        let program = program.step();
        let program = program.step();
        let _program = program.step();
        //let _program: ! = _program.step();
    }

    #[test]
    fn hello_world_loop() {
        // Assemble a type-level comment string
        type ThisProgramPrintsHelloWorldInAnInfiniteLoop = String![
            T h i s
            Space
            p r o g r a m
            p r i n t s
            h e l l o
            w o r l d
            i n
            a n
            i n f i n i t e
            l o o p
            Period
        ];

        let _remark: ThisProgramPrintsHelloWorldInAnInfiniteLoop;

        // Assemble a type-level "Hello, World!" string
        type HelloWorld = String![
            H e l l o
            Comma
            Space
            W o r l d
            Exclamation
        ];

        let _print: HelloWorld;

        // Type-level BASIC program
        type HelloWorldLoopProgram = Remark<
            ThisProgramPrintsHelloWorldInAnInfiniteLoop,
            Label<U10, Print<HelloWorld, Label<U20, Goto<U10>>>>,
        >;

        // Manually evaluate a fixed number of steps
        let program = basic::<HelloWorldLoopProgram>();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let program = program.step();
        let _program = program.step();
    }
}
