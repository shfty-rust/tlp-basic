use std::marker::PhantomData;

// Peano encoding for natural numbers
pub struct Z;
pub struct U<N>(PhantomData<N>);

macro_rules ! impl_peano {
    ($head:ty => $ident:ident $(,)?) => {
        pub type $ident = $head;
    };

    ($head:ty => $ident:ident, $($tt:tt)*) => {
        pub type $ident = $head;
        impl_peano!(U<$head> => $($tt)*);
    };
}

impl_peano![
    Z => U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20,
];

// Trait for program-level value access
pub trait Number {
    const VALUE: usize;
}

impl Number for Z {
    const VALUE: usize = 0;
}

impl<N> Number for U<N>
where
    N: Number,
{
    const VALUE: usize = N::VALUE + 1;
}

// Type level addition
pub trait AddOp<N> {
    type Add;
}

impl AddOp<Z> for Z {
    type Add = Z;
}

impl<N> AddOp<Z> for U<N> {
    type Add = U<N>;
}

impl<N> AddOp<U<N>> for Z {
    type Add = U<N>;
}

impl<L, R> AddOp<U<R>> for U<L>
where
    L: AddOp<U<R>>,
{
    type Add = U<L::Add>;
}

pub type Add<L, R> = <L as AddOp<R>>::Add;

// Type level subtraction
pub trait SubOp<N> {
    type Sub;
}

impl SubOp<Z> for Z {
    type Sub = Z;
}

impl<N> SubOp<Z> for U<N> {
    type Sub = U<N>;
}

impl<N> SubOp<U<Z>> for U<N> {
    type Sub = N;
}

impl<L, R> SubOp<U<U<R>>> for U<L>
where
    L: SubOp<U<R>>,
{
    type Sub = L::Sub;
}

pub type Sub<L, R> = <L as SubOp<R>>::Sub;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_level_add() {
        assert!(std::any::TypeId::of::<Add<U0, U0>>() == std::any::TypeId::of::<U0>());
        assert!(std::any::TypeId::of::<Add<U1, U0>>() == std::any::TypeId::of::<U1>());
        assert!(std::any::TypeId::of::<Add<U0, U1>>() == std::any::TypeId::of::<U1>());
        assert!(std::any::TypeId::of::<Add<U1, U1>>() == std::any::TypeId::of::<U2>());
        assert!(std::any::TypeId::of::<Add<U2, U1>>() == std::any::TypeId::of::<U3>());
        assert!(std::any::TypeId::of::<Add<U1, U2>>() == std::any::TypeId::of::<U3>());
        assert!(std::any::TypeId::of::<Add<U2, U2>>() == std::any::TypeId::of::<U4>());
    }

    #[test]
    fn type_level_sub() {
        assert!(std::any::TypeId::of::<Sub<U0, U0>>() == std::any::TypeId::of::<U0>());

        assert!(std::any::TypeId::of::<Sub<U1, U0>>() == std::any::TypeId::of::<U1>());
        assert!(std::any::TypeId::of::<Sub<U1, U1>>() == std::any::TypeId::of::<U0>());

        assert!(std::any::TypeId::of::<Sub<U2, U0>>() == std::any::TypeId::of::<U2>());
        assert!(std::any::TypeId::of::<Sub<U2, U1>>() == std::any::TypeId::of::<U1>());
        assert!(std::any::TypeId::of::<Sub<U2, U2>>() == std::any::TypeId::of::<U0>());

        assert!(std::any::TypeId::of::<Sub<U3, U0>>() == std::any::TypeId::of::<U3>());
        assert!(std::any::TypeId::of::<Sub<U3, U1>>() == std::any::TypeId::of::<U2>());
        assert!(std::any::TypeId::of::<Sub<U3, U2>>() == std::any::TypeId::of::<U1>());
        assert!(std::any::TypeId::of::<Sub<U3, U3>>() == std::any::TypeId::of::<U0>());

        assert!(std::any::TypeId::of::<Sub<U4, U0>>() == std::any::TypeId::of::<U4>());
        assert!(std::any::TypeId::of::<Sub<U4, U1>>() == std::any::TypeId::of::<U3>());
        assert!(std::any::TypeId::of::<Sub<U4, U2>>() == std::any::TypeId::of::<U2>());
        assert!(std::any::TypeId::of::<Sub<U4, U3>>() == std::any::TypeId::of::<U1>());
        assert!(std::any::TypeId::of::<Sub<U4, U4>>() == std::any::TypeId::of::<U0>());
    }
}
