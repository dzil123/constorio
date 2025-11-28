use std::{
    marker::PhantomData,
    ops::{Add, Div},
};

pub use typenum::{self, NonZero, Unsigned, consts::*};

use crate::private::New;

pub struct Tick<N: Unsigned>(PhantomData<N>);
impl_new!(Tick, N, Unsigned);
pub type NewTick = Tick<U0>;

pub trait Resource: private::Sealed {
    type MiningTicks: Unsigned + NonZero; // time to mine one of resource
}
pub struct Bundle<R: Resource, N: Unsigned>(PhantomData<(R, N)>);
impl_new!(Bundle, R, Resource, N, Unsigned);

pub struct Iron(PhantomData<()>);
pub type IronBundle<N> = Bundle<Iron, N>;

impl private::Sealed for Iron {}
impl Resource for Iron {
    type MiningTicks = U2;
}

pub struct Miner<R: Resource>(PhantomData<R>);
impl_new!(Miner, R, Resource);
pub type NewMiner<R> = Miner<R>;

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

impl<R: Resource> Miner<R> {
    pub fn mine_for_duration<BeforeTicks: Addable<Duration>, Duration: Divable<R::MiningTicks>>(
        self,
        _: Tick<BeforeTicks>,
    ) -> (Tick<BeforeTicks::Sum>, Self, Bundle<R, Duration::Quotient>) {
        (New::new(), New::new(), New::new())
    }
}

type GameFunction<N> = fn(NewTick, NewMiner<Iron>) -> (Tick<N>, Bundle<Iron, U5>);
pub fn run<N: Unsigned>(func: GameFunction<N>) -> usize {
    let _: (Tick<N>, Bundle<Iron, U5>) = (func)(New::new(), NewMiner::new());
    println!("successfully completed game in {} ticks", N::USIZE);
    N::USIZE
}

mod private {
    pub trait Sealed {}

    pub(crate) trait New {
        fn new() -> Self;
    }
    #[macro_export]
    macro_rules! impl_new {
        ($T: tt, $($Generic:tt, $Bound:tt),+) => {
            impl<$($Generic : $Bound),*> $crate::private::New for $T<$($Generic,)*> {
                fn new() -> Self {
                    Self(PhantomData)
                }
            }
        }
    }
}
