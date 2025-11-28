use backend::*;

fn main() {
    run(my_game);
    run(my_game2);
    run2(my_game3);
    //const X: usize = run_const!(my_game);
    //println!("ran const in {X} ticks");
}

#[unsafe(no_mangle)]
unsafe extern "C" fn foobar() -> usize {
    run(my_game)
}

const fn my_game(tick: NewTick, miner: NewMiner<Iron>) -> (Bundle<Iron, U5, impl Unsigned>,) {
    let (new_tick, miner, bundle1) = miner.mine_for_duration::<U10>(&tick);
    (bundle1,)
}

const fn my_game2(tick: NewTick, miner: NewMiner<Iron>) -> (Bundle<Iron, U5, impl Unsigned>,) {
    let (tick1, miner1, bundle1) = miner.mine_for_duration::<U6>(&tick);
    let (tick2, miner2, bundle2) = miner1.mine_for_duration::<U4>(&tick.ffwd::<U6>());
    let (tick3, miner3, bundle3) = miner2.mine_for_duration::<U0>(&tick2);
    (bundle2.ffwd::<U0>().combine(bundle1.ffwd::<U4>()).combine(bundle3),)
}

const fn my_game3(tick: NewTick, miner1: NewMiner<Iron>, miner2: NewMiner<Iron>) -> (Bundle<Iron, U5, impl Unsigned>,) {
    let (tick1, miner1, bundle1) = miner1.mine_for_duration::<U6>(&tick);
    let (tick2, miner2, bundle2) = miner2.mine_for_duration::<U4>(&tick);
    (bundle2.ffwd::<U2>().combine(bundle1.ffwd::<U0>()),)
}
