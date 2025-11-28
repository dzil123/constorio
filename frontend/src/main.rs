#![allow(unused)]

use backend::*;

fn main() {
    Tutorial::run(tutorial);
    Tutorial::run(tutorial2);
    Game::run(game_bad);
    Game::run(game_ok);
    Game::run(game_better);
    Game::run(game_best);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn foobar() -> usize {
    Game::run_quiet(game_best)
}

struct Tutorial;
impl Scenario for Tutorial {
    type StartItems = NewMiner<IronOre>;
    type EndResource = IronOre;
    type EndResourceCount = U5;
}

struct Game;
impl Scenario for Game {
    type StartItems = (NewMiner<IronOre>, NewFurnace, NewFurnace);
    type EndResource = IronIngot;
    type EndResourceCount = U10;
}

const fn tutorial(
    miner: ScenarioStartItems<Tutorial>,
    log: impl Log,
) -> (ScenarioEndBundle<Tutorial, impl Unsigned>, impl Log) {
    let (miner, iron_ore, log) = miner.mine_for_duration::<U10, _>(log);
    (iron_ore, log)
}

const fn tutorial2(
    miner: ScenarioStartItems<Tutorial>,
    log: impl Log,
) -> (ScenarioEndBundle<Tutorial, impl Unsigned>, impl Log) {
    let (miner1, bundle1, log) = miner.mine_for_duration::<U6, _>(log);
    let (miner2, bundle2, log) = miner1.mine_for_duration::<U4, _>(log);
    let (miner3, bundle3, log) = miner2.mine_for_duration::<U0, _>(log);
    (bundle1.combine(bundle2).combine(bundle3), log)
}

const fn game_bad(
    (miner, furnace, _): ScenarioStartItems<Game>,
    log: impl Log,
) -> (ScenarioEndBundle<Game, impl Unsigned>, impl Log) {
    let (miner, iron_ore, log) = miner.mine_for_duration::<U10, _>(log);
    let (furnace, iron_ingot, log) = furnace.smelt_all(iron_ore, log);
    (iron_ingot, log)
}

const fn game_ok(
    (miner, furnace1, furnace2): ScenarioStartItems<Game>,
    log: impl Log,
) -> (ScenarioEndBundle<Game, impl Unsigned>, impl Log) {
    let (miner, iron_ore1, log) = miner.mine_for_duration::<U4, _>(log);
    let (miner, iron_ore2, log) = miner.mine_for_duration::<U6, _>(log);
    let (furnace, iron_ingot2, log) = furnace2.smelt_all(iron_ore2, log);
    let (furnace, iron_ingot1, log) = furnace1.smelt_all(iron_ore1, log);
    (iron_ingot1.combine(iron_ingot2), log)
}

const fn game_better(
    (miner, furnace1, furnace2): ScenarioStartItems<Game>,
    log: impl Log,
) -> (ScenarioEndBundle<Game, impl Unsigned>, impl Log) {
    let (miner, iron_ore1, log) = miner.mine_for_duration::<U2, _>(log);
    let (miner, iron_ore2, log) = miner.mine_for_duration::<U2, _>(log);
    let (miner, iron_ore3, log) = miner.mine_for_duration::<U2, _>(log);
    let (miner, iron_ore4, log) = miner.mine_for_duration::<U2, _>(log);
    let (miner, iron_ore5, log) = miner.mine_for_duration::<U2, _>(log);

    let (furnace1, iron_ingot1, log) = furnace1.smelt_all(iron_ore1, log);
    let (furnace1, iron_ingot2, log) = furnace1.smelt_all(iron_ore2, log);
    let (furnace1, iron_ingot3, log) = furnace1.smelt_all(iron_ore3, log);
    let (furnace1, iron_ingot4, log) = furnace1.smelt_all(iron_ore4, log);
    let (furnace1, iron_ingot5, log) = furnace1.smelt_all(iron_ore5, log);
    (
        iron_ingot1
            .combine(iron_ingot2)
            .combine(iron_ingot3)
            .combine(iron_ingot4)
            .combine(iron_ingot5),
        log,
    )
}

const fn game_best(
    (miner, furnace1, furnace2): ScenarioStartItems<Game>,
    log: impl Log,
) -> (ScenarioEndBundle<Game, impl Unsigned>, impl Log) {
    let (miner, iron_ore1, log) = miner.mine_for_duration::<U2, _>(log);
    let (miner, iron_ore2, log) = miner.mine_for_duration::<U2, _>(log);
    let (miner, iron_ore3, log) = miner.mine_for_duration::<U2, _>(log);
    let (miner, iron_ore4, log) = miner.mine_for_duration::<U2, _>(log);
    let (miner, iron_ore5, log) = miner.mine_for_duration::<U2, _>(log);

    let (furnace1, iron_ingot1, log) = furnace1.smelt_all(iron_ore1, log);
    let (furnace2, iron_ingot2, log) = furnace2.smelt_all(iron_ore2, log);
    let (furnace1, iron_ingot3, log) = furnace1.smelt_all(iron_ore3, log);
    let (furnace2, iron_ingot4, log) = furnace2.smelt_all(iron_ore4, log);
    let (furnace1, iron_ingot5, log) = furnace1.smelt_all(iron_ore5, log);
    (
        iron_ingot1
            .combine(iron_ingot2)
            .combine(iron_ingot3)
            .combine(iron_ingot4)
            .combine(iron_ingot5),
        log,
    )
}
