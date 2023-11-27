use crate::read;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    /// Ore costs [x] ore
    ore: i32,
    /// Clay costs [x] ore
    clay: i32,
    /// Obby costs [x] ore and [x] clay
    obby: (i32, i32),
    /// Geode costs [x] ore and [x] obby
    geode: (i32, i32),
}

pub fn run() {
    let file = read!();

    // note: blueprint ids start at 1
    let blueprints: Vec<Blueprint> = file
        .map(|line| {
            let info_data = line.split_once(": ").unwrap().1;

            // each [robot] costs [x]
            let mut costs = info_data
                .split(". ")
                // costs {x ore and ...}
                .map(|sent| sent.split_once("costs ").unwrap().1.split(" "));

            // [x] "ore"
            let ore = costs.next().unwrap().next().unwrap().parse().unwrap();
            let clay = costs.next().unwrap().next().unwrap().parse().unwrap();

            // [x] "ore" "and" [y] "clay"
            let mut obby_costs = costs.next().unwrap();
            let obby_ore = obby_costs.next().unwrap().parse().unwrap();
            let obby_clay = obby_costs.skip(2).next().unwrap().parse().unwrap();

            let mut geode_costs = costs.next().unwrap();
            let geode_ore = geode_costs.next().unwrap().parse().unwrap();
            let geode_obby = geode_costs.skip(2).next().unwrap().parse().unwrap();

            Blueprint {
                ore,
                clay,
                obby: (obby_ore, obby_clay),
                geode: (geode_ore, geode_obby),
            }
        })
        .collect();

    let bp = blueprints[0];
    let state = State::new(bp);

    println!("{}", dfs(state, 24));
}

#[derive(Clone, Copy)]
struct State {
    blueprint: Blueprint,
    robots_ore: i32,
    robots_clay: i32,
    robots_obby: i32,
    robots_geode: i32,
    ore: i32,
    clay: i32,
    obby: i32,
    geode: i32,
}

impl State {
    fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            robots_ore: 1,
            robots_clay: 0,
            robots_obby: 0,
            robots_geode: 0,
            ore: 0,
            clay: 0,
            obby: 0,
            geode: 0,
        }
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("ore", &self.ore)
            .field("clay", &self.clay)
            .field("obby", &self.obby)
            .field("geode", &self.geode)
            // .field("ore_robots", &self.robots_ore)
            // .field("clay_robots", &self.robots_clay)
            // .field("obby_robots", &self.robots_obby)
            // .field("geode_robots", &self.robots_geode)
            .finish()
    }
}

/*

*/

/// time = 24
///
/// just bought is = max amt of ore of obsidian robot, to give chance to buy those
fn dfs(mut state: State, time: i32) -> i32 {
    // base case: the output
    if time == 0 {
        return state.geode;
    }

    state.ore += state.robots_ore;
    state.clay += state.robots_clay;
    state.obby += state.robots_obby;
    state.geode += state.robots_geode;

    // println!("{state:?} {:?}", state.blueprint);

    let mut max = state.geode;

    if state.ore >= state.blueprint.obby.0 && state.clay >= state.blueprint.obby.1 {
        let mut state = state;
        state.ore -= state.blueprint.obby.0;
        state.clay -= state.blueprint.obby.1;
        max = max.max(dfs(state, time - 1));
    }

    if state.ore >= state.blueprint.geode.0 && state.obby >= state.blueprint.geode.1 {
        let mut state = state;
        state.ore -= state.blueprint.geode.0;
        state.obby -= state.blueprint.geode.1;
        max = max.max(dfs(state, time - 1));
    }

    if state.ore >= state.blueprint.ore {
        // epic copy method
        let mut state = state;
        state.ore -= state.blueprint.ore;
        state.robots_ore += 1;
        max = max.max(dfs(state, time - 1));
    }

    if state.ore >= state.blueprint.clay {
        let mut state = state;
        state.clay -= state.blueprint.clay;
        state.robots_clay += 1;
        max = max.max(dfs(state, time - 1));
    }

    // println!("going on... with state {state:?}");
    // input!();

    max = max.max(dfs(state, time - 1));

    max
}
