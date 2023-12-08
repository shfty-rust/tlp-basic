use crate::string::StringTrait;
use std::marker::PhantomData;

// Type-level characters
macro_rules! impl_char {
    ($char:expr => $ty:ident) => {
        pub struct $ty;

        impl StringTrait for $ty {
            fn string() -> String {
                $char.to_string()
            }
        }
    };
}

impl_char!("a" => a);
impl_char!("b" => b);
impl_char!("c" => c);
impl_char!("d" => d);
impl_char!("e" => e);
impl_char!("f" => f);
impl_char!("g" => g);
impl_char!("h" => h);
impl_char!("i" => i);
impl_char!("j" => j);
impl_char!("k" => k);
impl_char!("l" => l);
impl_char!("m" => m);
impl_char!("n" => n);
impl_char!("o" => o);
impl_char!("p" => p);
impl_char!("q" => q);
impl_char!("r" => r);
impl_char!("s" => s);
impl_char!("t" => t);
impl_char!("u" => u);
impl_char!("v" => v);
impl_char!("w" => w);
impl_char!("x" => x);
impl_char!("y" => y);
impl_char!("z" => z);

impl_char!("A" => A);
impl_char!("B" => B);
impl_char!("C" => C);
impl_char!("D" => D);
impl_char!("E" => E);
impl_char!("F" => F);
impl_char!("G" => G);
impl_char!("H" => H);
impl_char!("I" => I);
impl_char!("J" => J);
impl_char!("K" => K);
impl_char!("L" => L);
impl_char!("M" => M);
impl_char!("N" => N);
impl_char!("O" => O);
impl_char!("P" => P);
impl_char!("Q" => Q);
impl_char!("R" => R);
impl_char!("S" => S);
impl_char!("T" => T);
impl_char!("U" => U);
impl_char!("V" => V);
impl_char!("W" => W);
impl_char!("X" => X);
impl_char!("Y" => Y);
impl_char!("Z" => Z);

impl_char!(" " => Space);
impl_char!("." => Period);
impl_char!("," => Comma);
impl_char!("!" => Exclamation);
