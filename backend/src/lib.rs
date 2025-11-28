use std::{marker::PhantomData, ops::*};

pub use typenum::{self, NonZero, Unsigned, consts::*, operator_aliases::*, type_operators::*};

use crate::new::New;

mod new;

macro_rules! make_fn {
    {
        $(<$T:ty, $U:ty> $(,)?)*
        $(Diff<$Tsub:ty, $Usub:ty> $(,)?)*
        $(Quot<$Tdiv:ty, $Udiv:ty> $(,)?)*
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
            $Tsub: Unsigned + Sub<$Usub>,
            $Usub: Unsigned,
            Diff<$Tsub, $Usub>: Unsigned,
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

    make_fn! {
        Diff<N, N2>;
        pub const fn split<N2>(self) -> (Bundle<R, N2, TS>, Bundle<R, Diff<N, N2>, TS>)
    }
}

pub struct IronOre;
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

pub struct IronIngot;
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

pub struct Furnace<TS: Unsigned>(PhantomData<TS>);
impl_new!(Furnace, TS, Unsigned);
pub type NewFurnace = Furnace<U0>;

impl<TS: Unsigned> Furnace<TS> {
    make_fn! {
        <TS, Duration>;
        pub const fn ffwd<Duration>(self) -> Furnace<Sum<TS, Duration>>
    }

    make_fn! {
        <TS, TS2>, <NIn, R::SmeltTicks>,
        <Maximum<TS, TS2>, Prod<NIn, R::SmeltTicks>>,
        <Quot<NIn, R::SmeltInCount>, R::SmeltOutCount>
        Quot<NIn, R::SmeltInCount>;
        pub const fn smelt_all<R: CanSmelt, NIn, TS2>(self, _: Bundle<R, NIn, TS2>) -> (
            Furnace<
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

pub trait Scenario {
    #[expect(private_bounds)]
    type StartItems: New;
    type EndResource: Resource;
    type EndResourceCount: Unsigned;

    fn run<TS: Unsigned>(func: GameFunction<Self, TS>) -> usize {
        let _: ScenarioEndBundle<Self, TS> = (func)(New::NEW);
        println!(
            "completed scenario {} in {} ticks",
            std::any::type_name::<Self>(),
            TS::USIZE
        );
        TS::USIZE
    }

    fn run_quiet<TS: Unsigned>(func: GameFunction<Self, TS>) -> usize {
        let _: ScenarioEndBundle<Self, TS> = (func)(New::NEW);
        TS::USIZE
    }
}
pub type ScenarioStartItems<S> = <S as Scenario>::StartItems;
pub type ScenarioEndBundle<S, TS> =
    Bundle<<S as Scenario>::EndResource, <S as Scenario>::EndResourceCount, TS>;

pub type GameFunction<S, TS> = fn(ScenarioStartItems<S>) -> ScenarioEndBundle<S, TS>;

mod private {
    pub trait Sealed {}
}
