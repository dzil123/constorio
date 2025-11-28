use backend::*;

fn main() {
    run(my_game);
}

fn my_game(tick: NewTick, miner: NewMiner<Iron>) -> (Tick<impl Unsigned>, Bundle<Iron, U5>) {
    let (tick, _miner, bundle) = miner.mine_for_duration::<_, U10>(tick);
    (tick, bundle)
}
