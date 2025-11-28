use std::marker::PhantomData;

pub use typenum;
use typenum::{NonZero, Unsigned, consts::*};

use crate::private::New;

pub struct Tick<N: Unsigned>(PhantomData<N>);
pub type NewTick = Tick<U0>;
impl_new!(NewTick);

pub trait Resource: private::Sealed {
    type MiningTicks: Unsigned + NonZero; // time to mine one of resource
}
pub struct Bundle<R: Resource, N: Unsigned>(PhantomData<(R, N)>);
impl<R: Resource, N: Unsigned> Bundle<R, N> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

pub struct Iron(PhantomData<()>);
pub type IronBundle<N> = Bundle<Iron, N>;

impl private::Sealed for Iron{}
impl Resource for Iron {
    type MiningTicks = U2;
}

pub struct Miner<R: Resource, Tick: Unsigned>(PhantomData<(R, Tick)>);
impl<R: Resource, Tick: Unsigned> Miner<R, Tick> {
    fn new() -> Self {
        Self(PhantomData)
    }
}
pub type NewMiner<R> = Miner<R, U0>;



impl <R: Resource, Tick: Unsigned> Miner<R, Tick> {
    pub fn mine_for_duration<PrevTick: Unsigned, Duration: Unsigned>(Self, )
}

type GameFunction<N> = fn(NewTick, NewMiner<Iron>) -> (Tick<N>, Bundle<Iron, U5>);
pub fn run<N: Unsigned>(func: GameFunction<N>) {
    let _: (Tick<N>, Bundle<Iron, U5>) = (func)(New::new(), NewMiner::new());
    println!("successfully completed game in {} ticks", N::USIZE);
}

mod private {
    pub trait Sealed {}

    pub(crate) trait New {
        fn new() -> Self;
    }
    #[macro_export]
    macro_rules! impl_new {
        ($T:ty) => {
            impl crate::private::New for $T {
                fn new() -> Self {
                    Self(PhantomData)
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
}
