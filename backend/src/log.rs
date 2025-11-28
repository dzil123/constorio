use std::any::type_name;
use std::marker::PhantomData;

use typenum::Unsigned;

use crate::{CanMine, CanSmelt, impl_new, private};

pub struct Nil(PhantomData<()>);
impl private::Sealed for Nil {}
// impl_new!(Nil,);
pub(crate) const NIL: Nil = Nil(PhantomData);

#[allow(private_bounds)]
pub struct Cons<L: Log, T: Event>(PhantomData<(L, T)>);
impl<L: Log, T: Event> private::Sealed for Cons<L, T> {}
impl_new!(Cons, L, Log, T, Event);
// TODO how to prevent user from Newing Cons

pub trait Log: private::Sealed {
    fn print();
}
impl Log for Nil {
    fn print() {}
}
impl<L: Log, T: Event> Log for Cons<L, T> {
    fn print() {
        L::print();
        T::print();
    }
}

pub(crate) trait Event {
    fn print();
}

// #[derive(Clone, Copy)]
pub struct MineEvent<R: CanMine, Duration: Unsigned, TS: Unsigned>(PhantomData<(R, Duration, TS)>);
impl<R: CanMine, Duration: Unsigned, TS: Unsigned> Event for MineEvent<R, Duration, TS> {
    fn print() {
        println!(
            "{}: mined {} for {}ts",
            TS::USIZE,
            type_name::<R>(),
            Duration::USIZE
        );
    }
}

#[derive(Clone, Copy)]
pub struct SmeltEvent<R: CanSmelt, Duration: Unsigned, TS: Unsigned>(
    PhantomData<(R, Duration, TS)>,
);
impl<R: CanSmelt, Duration: Unsigned, TS: Unsigned> Event for SmeltEvent<R, Duration, TS> {
    fn print() {
        println!(
            "{}: smelt {} for {}ts",
            TS::USIZE,
            type_name::<R>(),
            Duration::USIZE
        );
    }
}
