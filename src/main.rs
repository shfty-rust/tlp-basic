pub mod basic;
pub mod boolean;
pub mod char;
pub mod cons;
pub mod number;
pub mod string;

fn main() {
    use crate::number::*;
    use crate::basic::Run;

    // Basic Hello World
    type ThisIsABasicHelloWorldProgram = String![
        T h i s
        Space
        i s
        Space
        a
        Space
        b a s i c
        Space
        h e l l o
        Space
        w o r l d
        Space
        p r o g r a m
        Period
    ];

    type HelloWorld = String![
        H e l l o
        Comma
        Space
        W o r l d
        Exclamation
    ];

    type HelloWorldProgram = Basic! {
        REM ThisIsABasicHelloWorldProgram
        CLS
        PRINT HelloWorld
        END
    };

    let program = basic::basic::<HelloWorldProgram>();
    //program.run();

    // Infinite Hello World loop
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

    type HelloWorldLoop = Basic! {
        REM ThisProgramPrintsHelloWorldInAnInfiniteLoop
        CLS
        U10: PRINT HelloWorld
        U20: GOTO U10
    };

    let program = basic::basic::<HelloWorldLoop>();
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
    let program = program.step();
    let program = program.step();
    let program = program.step();
    let program = program.step();
    let _program = program.step();

    // Let binding
    struct ThisProgramBindsVariables;
    type Foo = U0;
    type Bar = U1;
    type Baz = U2;
    type LetBindingProgram = Basic! {
        REM ThisProgramBindsVariables
        LET Foo = U1
        LET Bar = U4
        LET Baz = U8
        END
    };

    let program = basic::basic::<LetBindingProgram>();
    program.run();
}
