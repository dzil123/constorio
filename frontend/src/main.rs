#![allow(unused)]

use backend::*;

fn main() {
    Tutorial::run(tutorial);
    Tutorial::run(tutorial2);
    Game::run(game_bad);
    Game::run(game_better);
    // Game::run(game_best);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn foobar() -> usize {
    Tutorial::run(tutorial)
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
) -> ScenarioEndBundle<Tutorial, impl Unsigned> {
    let (miner, iron_ore) = miner.mine_for_duration::<U10>();
    iron_ore
}

const fn tutorial2(
    miner: ScenarioStartItems<Tutorial>,
) -> ScenarioEndBundle<Tutorial, impl Unsigned> {
    let (miner1, bundle1) = miner.mine_for_duration::<U6>();
    let (miner2, bundle2) = miner1.mine_for_duration::<U4>();
    let (miner3, bundle3) = miner2.mine_for_duration::<U0>();
    bundle1.combine(bundle2).combine(bundle3)
}

const fn game_bad(
    (miner, furnace, _): ScenarioStartItems<Game>,
) -> ScenarioEndBundle<Game, impl Unsigned> {
    let (miner, iron_ore) = miner.mine_for_duration::<U10>();
    let (furnace, iron_ingot) = furnace.smelt_all(iron_ore);
    iron_ingot
}

const fn game_better(
    (miner, furnace1, furnace2): ScenarioStartItems<Game>,
) -> ScenarioEndBundle<Game, impl Unsigned> {
    let (miner, iron_ore1) = miner.mine_for_duration::<U4>();
    let (miner, iron_ore2) = miner.mine_for_duration::<U6>();
    let (furnace, iron_ingot2) = furnace2.smelt_all(iron_ore2);
    let (furnace, iron_ingot1) = furnace1.smelt_all(iron_ore1);
    iron_ingot1.combine(iron_ingot2)
}

