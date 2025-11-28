use std::{marker::PhantomData, ops::*};

pub use typenum::{self, NonZero, Unsigned, consts::*, operator_aliases::*, type_operators::*};

use crate::new::New;

mod new;

macro_rules! make_fn {
    {
        $(<$T:ty, $U:ty>),* $(,)?
        $(Quot<$Tdiv:ty, $Udiv:ty>),*
        ; $($stub:tt)*
    } => {
        $($stub)*
        where
        $(
            $T: Unsigned + Add<$U> + Max<$U> + Mul<$U>,
            $U: Unsigned,
            Sum<$T, $U>: Unsigned,
            Maximum<$T, $U>: Unsigned,
            Prod<$T, $U>: Unsigned,
        )*
        $(
            $Tdiv: Unsigned + Div<$Udiv>,
            $Udiv: Unsigned,
            Quot<$Tdiv, $Udiv>: Unsigned,
        )*
        { New::NEW }
    }
}

pub trait Resource: private::Sealed {}
pub trait CanMine: Resource {
    type MiningTicks: Unsigned + NonZero; // time to mine one of resource
}
pub trait CanSmelt: Resource {
    type SmeltTicks: Unsigned + NonZero;
    type SmeltInCount: Unsigned + NonZero;
    type SmeltOutCount: Unsigned + NonZero;
    type SmeltOutput: Resource;
}

// TS means Timestamp
pub struct Bundle<R: Resource, N: Unsigned, TS: Unsigned>(PhantomData<(R, N, TS)>);
impl_new!(Bundle, R, Resource, N, Unsigned, TS, Unsigned);

impl<R: Resource, N: Unsigned, TS: Unsigned> Bundle<R, N, TS> {
    make_fn! {
        <TS, Duration>;
        pub const fn ffwd<Duration>(self) -> Bundle<R, N, Sum<TS, Duration>>
    }

    make_fn! {
        <N, N2>, <TS, TS2>;
        pub const fn combine<N2, TS2>(self, _: Bundle<R, N2, TS2>) -> Bundle<R, Sum<N, N2>, Maximum<TS, TS2>>
    }
}

pub struct IronOre(PhantomData<()>);
impl private::Sealed for IronOre {}
impl Resource for IronOre {}
impl CanMine for IronOre {
    type MiningTicks = U2;
}
impl CanSmelt for IronOre {
    type SmeltTicks = U3;
    type SmeltInCount = U1;
    type SmeltOutCount = U2;
    type SmeltOutput = IronIngot;
}

pub struct IronIngot(PhantomData<()>);
impl private::Sealed for IronIngot {}
impl Resource for IronIngot {}

pub struct Miner<R: CanMine, TS: Unsigned>(PhantomData<(R, TS)>);
impl_new!(Miner, R, CanMine, TS, Unsigned);
pub type NewMiner<R> = Miner<R, U0>;

impl<R: CanMine, TS: Unsigned> Miner<R, TS> {
    make_fn! {
        <TS, Duration>;
        pub const fn ffwd<Duration>(self) -> Miner<R, Sum<TS, Duration>>
    }

    make_fn! {
        <TS, Duration>,
        Quot<Duration, R::MiningTicks>;
        pub const fn mine_for_duration<Duration>(self) -> (
            Miner<R, Sum<TS, Duration>>,
            Bundle<R, Quot<Duration, R::MiningTicks>, Sum<TS, Duration>>,
        )
    }
}

pub struct Furnace<R: CanSmelt, TS: Unsigned>(PhantomData<(R, TS)>);
impl_new!(Furnace, R, CanSmelt, TS, Unsigned);
impl<R: CanSmelt, TS: Unsigned> Furnace<R, TS> {
    make_fn! {
        <TS, TS2>, <NIn, R::SmeltTicks>,
        <Maximum<TS, TS2>, Prod<NIn, R::SmeltTicks>>,
        <Quot<NIn, R::SmeltInCount>, R::SmeltOutCount>
        Quot<NIn, R::SmeltInCount>;
        pub const fn smelt_all<NIn, TS2>(self, _: Bundle<R, NIn, TS2>) -> (
            Furnace<
                R,
                Sum<Maximum<TS, TS2>, Prod<NIn, R::SmeltTicks>>
            >,
            Bundle<
                R::SmeltOutput,
                Prod<Quot<NIn, R::SmeltInCount>, R::SmeltOutCount>,
                Sum<Maximum<TS, TS2>, Prod<NIn, R::SmeltTicks>>
            >,
        )
    }
}

type GameFunction<TS> = fn(NewMiner<IronOre>) -> (Bundle<IronOre, U5, TS>,);
pub fn run<TS: Unsigned>(func: GameFunction<TS>) -> usize {
    let _: (Bundle<IronOre, U5, TS>,) = (func)(New::NEW);
    println!("successfully completed game in {} ticks", TS::USIZE);
    TS::USIZE
}

type GameFunction2<TS> = fn(NewMiner<IronOre>, NewMiner<IronOre>) -> (Bundle<IronOre, U5, TS>,);
pub fn run2<TS: Unsigned>(func: GameFunction2<TS>) -> usize {
    let _: (Bundle<IronOre, U5, TS>,) = (func)(New::NEW, New::NEW);
    println!(
        "successfully completed game with 2 miners in {} ticks",
        TS::USIZE
    );
    TS::USIZE
}

mod private {
    pub trait Sealed {}
}
