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

const fn my_game(miner: NewMiner<Iron>) -> (Bundle<Iron, U5, impl Unsigned>,) {
    let (miner, bundle1) = miner.mine_for_duration::<U10>();
    (bundle1,)
}

const fn my_game2(miner: NewMiner<Iron>) -> (Bundle<Iron, U5, impl Unsigned>,) {
    let (miner1, bundle1) = miner.mine_for_duration::<U6>();
    let (miner2, bundle2) = miner1.mine_for_duration::<U4>();
    let (miner3, bundle3) = miner2.mine_for_duration::<U0>();
    (bundle2.ffwd::<U0>().combine(bundle1.ffwd::<U4>()).combine(bundle3),)
}

const fn my_game3(miner1: NewMiner<Iron>, miner2: NewMiner<Iron>) -> (Bundle<Iron, U5, impl Unsigned>,) {
    let (miner1, bundle1) = miner1.mine_for_duration::<U6>();
    let (miner2, bundle2) = miner2.mine_for_duration::<U4>();
    (bundle2.ffwd::<U2>().combine(bundle1.ffwd::<U0>()),)
}
