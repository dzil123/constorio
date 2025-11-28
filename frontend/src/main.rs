use backend::*;

fn main() {
    run(my_game);
    //const X: usize = run_const!(my_game);
    //println!("ran const in {X} ticks");
}

#[unsafe(no_mangle)]
unsafe extern "C" fn foobar() -> usize {
    run(my_game)
}

const fn my_game(tick: NewTick, miner: NewMiner<Iron>) -> (Tick<impl Unsigned>, Bundle<Iron, U5>) {
    let (tick, _miner, bundle) = miner.mine_for_duration::<_, U10>(tick);
    (tick, bundle)
}
