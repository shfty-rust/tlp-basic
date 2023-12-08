use crate::number::*;

#[macro_export]
macro_rules ! List {
    ($ty:ty, $($tt:tt)*) => {
        ($ty, $crate::List![$($tt)*])
    };

    ($ty:ty) => {
        ($ty, ())
    };

    () => {
        ()
    }
}

// Cons list push
pub trait PushOp<T> {
    type Push;
}

impl<CAR, CDR, T> PushOp<T> for (CAR, CDR)
where
    CDR: PushOp<T>,
{
    type Push = (CAR, CDR::Push);
}

impl<T> PushOp<T> for () {
    type Push = (T, ());
}

pub type Push<T, V> = <T as PushOp<V>>::Push;

// Cons list pop
pub trait PopOp {
    type Pop;
}

impl<CAR, CDR> PopOp for (CAR, CDR)
where
    CDR: PopOp,
{
    type Pop = (CAR, CDR::Pop);
}

impl<CAR> PopOp for (CAR, ()) {
    type Pop = ();
}

pub type Pop<T> = <T as PopOp>::Pop;

// Cons list index
pub trait IndexOp<N> {
    type Index;
}

impl<CAR, CDR> IndexOp<Z> for (CAR, CDR) {
    type Index = CAR;
}

impl<N, CAR, CDR> IndexOp<U<N>> for (CAR, CDR)
where
    CDR: IndexOp<N>,
{
    type Index = CDR::Index;
}

pub type Index<T, N> = <T as IndexOp<N>>::Index;

// Cons list remove
pub trait RemoveOp<N> {
    type Remove;
}

impl<CAR> RemoveOp<Z> for (CAR, ()) {
    type Remove = ();
}

impl<LCAR, RCAR, CDR> RemoveOp<Z> for (LCAR, (RCAR, CDR)) {
    type Remove = (RCAR, CDR);
}

impl<LCAR, RCAR, CDR> RemoveOp<U<Z>> for (LCAR, (RCAR, CDR)) {
    type Remove = (LCAR, CDR);
}

impl<N, LCAR, RCAR, CDR> RemoveOp<U<U<N>>> for (LCAR, (RCAR, CDR))
where
    CDR: RemoveOp<N>,
{
    type Remove = (LCAR, (RCAR, CDR::Remove));
}

pub type Remove<T, N> = <T as RemoveOp<N>>::Remove;

// Cons list insert
pub trait InsertOp<N, V> {
    type Insert;
}

impl<V> InsertOp<Z, V> for () {
    type Insert = (V, ());
}

impl<V, CAR, CDR> InsertOp<Z, V> for (CAR, CDR) {
    type Insert = (V, (CAR, CDR));
}

impl<N, V, CAR, CDR> InsertOp<U<N>, V> for (CAR, CDR)
where
    CDR: InsertOp<N, V>,
{
    type Insert = CDR::Insert;
}

pub type Insert<T, K, V> = <T as InsertOp<K, V>>::Insert;

// Cons list set
pub trait SetOp<N, V, D = ()> {
    type Set;
}

impl<V, D> SetOp<U0, V, D> for () {
    type Set = (V, ());
}

impl<N, V, D> SetOp<U<N>, V, D> for ()
where
    (): SetOp<N, V, D>,
{
    type Set = (D, <() as SetOp<N, V, D>>::Set);
}

impl<CAR, CDR, V, D> SetOp<U0, V, D> for (CAR, CDR) {
    type Set = (V, CDR);
}

impl<CAR, CDR, N, V, D> SetOp<U<N>, V, D> for (CAR, CDR)
where
    CDR: SetOp<N, V, D>,
{
    type Set = (CAR, <CDR as SetOp<N, V, D>>::Set);
}

pub type Set<T, N, V, D = ()> = <T as SetOp<N, V, D>>::Set;

// Cons map find
//
// Finds a (Key, Value) tuple by Key and outputs Value
//
// Limitation: Needs type parameter I (index) to differentiate impls
// Can be inferred at the callsite in some cases, but makes generic use difficult
pub trait MapFindOp<K, I> {
    type MapFind;
}

impl<K, V, CDR> MapFindOp<K, U0> for ((K, V), CDR) {
    type MapFind = V;
}

impl<K, N, CAR, CDR> MapFindOp<K, U<N>> for (CAR, CDR)
where
    CDR: MapFindOp<K, N>,
{
    type MapFind = CDR::MapFind;
}

pub type MapFind<T, K, I> = <T as MapFindOp<K, I>>::MapFind;

#[cfg(test)]
mod tests {
    use crate::cons::*;

    type List = Push<Push<Push<(), u32>, f32>, String>;

    #[test]
    fn cons_push() {
        // Cons list test types
        assert!(
            std::any::TypeId::of::<List>() == std::any::TypeId::of::<List![u32, f32, String]>()
        );
    }

    #[test]
    fn cons_pop() {
        assert!(std::any::TypeId::of::<Pop<List>>() == std::any::TypeId::of::<List![u32, f32]>());
        assert!(std::any::TypeId::of::<Pop<Pop<List>>>() == std::any::TypeId::of::<List![u32]>());
    }

    #[test]
    fn cons_index() {
        assert!(std::any::TypeId::of::<Index<List, U0>>() == std::any::TypeId::of::<u32>());
        assert!(std::any::TypeId::of::<Index<List, U1>>() == std::any::TypeId::of::<f32>());
        assert!(std::any::TypeId::of::<Index<List, U2>>() == std::any::TypeId::of::<String>());
    }

    #[test]
    fn cons_remove() {
        assert!(
            std::any::TypeId::of::<Remove<List, U0>>()
                == std::any::TypeId::of::<List![f32, String]>()
        );
        assert!(
            std::any::TypeId::of::<Remove<List, U1>>()
                == std::any::TypeId::of::<List![u32, String]>()
        );
        assert!(
            std::any::TypeId::of::<Remove<List, U2>>() == std::any::TypeId::of::<List![u32, f32]>()
        );
    }

    #[test]
    fn cons_insert() {
        type InsertList = Insert<Insert<Insert<(), U0, U1>, U0, U2>, U0, U4>;
        assert!(
            std::any::TypeId::of::<InsertList>() == std::any::TypeId::of::<List![U4, U2, U1]>()
        );
    }

    #[test]
    fn cons_set() {
        struct Foo;
        struct Bar;
        struct Baz;

        type SetFoo = Set<(), U9, Foo>;
        assert!(
            std::any::TypeId::of::<SetFoo>()
                == std::any::TypeId::of::<List![(), (), (), (), (), (), (), (), (), Foo]>()
        );

        type SetBar = Set<SetFoo, U1, Bar>;
        assert!(
            std::any::TypeId::of::<SetBar>()
                == std::any::TypeId::of::<List![(), Bar, (), (), (), (), (), (), (), Foo]>()
        );

        type SetBaz = Set<SetBar, U6, Baz>;
        assert!(
            std::any::TypeId::of::<SetBaz>()
                == std::any::TypeId::of::<List![(), Bar, (), (), (), (), Baz, (), (), Foo]>()
        );
    }

    #[test]
    fn cons_map_find() {
        struct Foo;
        struct Bar;
        struct Baz;

        type InsertList = Insert<Insert<Insert<(), U0, (Foo, U1)>, U0, (Bar, U2)>, U0, (Baz, U4)>;

        assert!(
            std::any::TypeId::of::<MapFind<InsertList, Foo, _>>() == std::any::TypeId::of::<U1>()
        );
        assert!(
            std::any::TypeId::of::<MapFind<InsertList, Bar, _>>() == std::any::TypeId::of::<U2>()
        );
        assert!(
            std::any::TypeId::of::<MapFind<InsertList, Baz, _>>() == std::any::TypeId::of::<U4>()
        );
    }
}
