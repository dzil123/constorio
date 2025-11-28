use std::{
    marker::PhantomData,
    ops::{Add, Div},
};

pub use typenum::{self, NonZero, Unsigned, consts::*};

use crate::new::New;

mod new;

pub trait Resource: private::Sealed {
    type MiningTicks: Unsigned + NonZero; // time to mine one of resource
}

// TS means Timestamp
pub struct Bundle<R: Resource, N: Unsigned, TS: Unsigned>(PhantomData<(R, N, TS)>);
impl_new!(Bundle, R, Resource, N, Unsigned, TS, Unsigned);

impl<R: Resource, N: Unsigned, TS: Unsigned> Bundle<R, N, TS> {
    pub const fn ffwd<Duration: Addable<TS>>(self) -> Bundle<R, N, Duration::Sum>
    {
        New::NEW
    }

    /// combine two bundles with same resource and timestamp but different N counts into the sum
    pub const fn combine<N2: Addable<N>>(self, _: Bundle<R, N2, TS>) -> Bundle<R, N2::Sum, TS>
    {
        New::NEW
    }
}

pub struct Iron(PhantomData<()>);
pub type IronBundle<N, TS> = Bundle<Iron, N, TS>;

impl private::Sealed for Iron {}
impl Resource for Iron {
    type MiningTicks = U2;
}

pub struct Miner<R: Resource, TS: Unsigned>(PhantomData<(R, TS)>);
impl_new!(Miner, R, Resource, TS, Unsigned);
pub type NewMiner<R> = Miner<R, U0>;

pub trait Addable<Rhs: Unsigned>: Unsigned {
    type Sum: Unsigned;
}
impl<T: Unsigned, U: Unsigned, V: Unsigned> Addable<U> for T
where
    T: Add<U, Output = V>,
{
    type Sum = V;
}

pub trait Divable<Rhs: Unsigned>: Unsigned {
    type Quotient: Unsigned;
}
impl<T: Unsigned, U: Unsigned, V: Unsigned> Divable<U> for T
where
    T: Div<U, Output = V>,
{
    type Quotient = V;
}

impl<R: Resource, TS: Unsigned> Miner<R, TS> {
    pub const fn ffwd<Duration: Addable<TS>>(self) -> Miner<R, Duration::Sum>
    {
        New::NEW
    }

    pub const fn mine_for_duration<Duration: Addable<TS> + Divable<R::MiningTicks>>(
        self
    ) -> (
        Miner<R, Duration::Sum>,
        Bundle<R, Duration::Quotient, Duration::Sum>,
    )
    {
        New::NEW
    }
}

type GameFunction<TS> = fn(NewMiner<Iron>) -> (Bundle<Iron, U5, TS>,);
pub fn run<TS: Unsigned>(func: GameFunction<TS>) -> usize {
    let _: (Bundle<Iron, U5, TS>,) = (func)(New::NEW);
    println!("successfully completed game in {} ticks", TS::USIZE);
    TS::USIZE
}

type GameFunction2<TS> = fn(NewMiner<Iron>, NewMiner<Iron>) -> (Bundle<Iron, U5, TS>,);
pub fn run2<TS: Unsigned>(func: GameFunction2<TS>) -> usize {
    let _: (Bundle<Iron, U5, TS>,) = (func)(New::NEW, New::NEW);
    println!("successfully completed game with 2 miners in {} ticks", TS::USIZE);
    TS::USIZE
}

mod private {
    pub trait Sealed {}
}
