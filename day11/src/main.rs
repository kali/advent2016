use itertools::Itertools;
use std::collections::BTreeSet;

#[derive(Hash, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Artefact {
    Gen(String),
    MicroChip(String),
}
use Artefact::*;

impl Artefact {
    fn is_mc(&self) -> bool {
        matches!(self, MicroChip(_))
    }

    fn metal(&self) -> &str {
        match self {
            Gen(m) => m,
            MicroChip(m) => m,
        }
    }
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
struct State {
    elevator: usize,
    floors: Vec<BTreeSet<Artefact>>,
}

impl State {
    fn valid(&self) -> bool {
        self.floors.iter().all(|f| {
            f.iter().all(|a| {
                a.is_mc()
                    || f.iter()
                        .all(|a| !a.is_mc() || f.contains(&Gen(a.metal().to_string())))
            })
        })
    }

    fn moves(&self) -> Vec<State> {
        let taking: Vec<Vec<Artefact>> = [1, 2]
            .into_iter()
            .flat_map(|n| {
                self.floors[self.elevator]
                    .iter()
                    .combinations(n)
                    .into_iter()
                    .map(|c| c.into_iter().cloned().collect())
            })
            .collect();
        let going: Vec<usize> = [-1, 1]
            .into_iter()
            .map(|x| ((self.elevator as isize + x).min(3).max(0) as usize))
            .filter(|x| *x != self.elevator)
            .collect();
        itertools::iproduct!(going, taking)
            .map(|(going, taking)| {
                let mut floors = self.floors.clone();
                floors[self.elevator].retain(|art| !taking.contains(art));
                floors[going].extend(taking);
                State {
                    elevator: going,
                    floors,
                }
            })
            .filter(|s| s.valid())
            .collect()
    }
}

fn parse(input: &str) -> State {
    let mut floors = vec![];
    for line in input.lines() {
        let mut arts = BTreeSet::new();
        let words: Vec<&str> = line.split(' ').collect();
        for w in 0..words.len() {
            if words[w].ends_with("-compatible") {
                arts.insert(MicroChip(words[w].split_once("-").unwrap().0.to_string()));
            } else if words[w].contains("generator") {
                arts.insert(Gen(words[w - 1].to_string()));
            }
        }
        floors.push(arts);
    }
    State {
        elevator: 0,
        floors,
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input, false));
    dbg!(run(&input, true));
}

fn run(input: &str, part2: bool) -> usize {
    let mut state = parse(input);
    if part2 {
        state.floors[0].extend([
            Gen("elerium".to_string()),
            MicroChip("elerium".to_string()),
            Gen("dilithium".to_string()),
            MicroChip("dilithium".to_string()),
        ]);
    }
    let path = pathfinding::directed::astar::astar(
        &state,
        |s| s.moves().into_iter().map(|s| (s, 1)),
        |s| s.floors[0..3].iter().map(|f| f.len()).sum::<usize>() / 2,
        |s| s.elevator == 3 && s.floors[0..3].iter().all(|f| f.is_empty()),
    )
    .unwrap();
    path.1
}

#[test]
fn t() {
    assert_eq!(run("The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.", false), 11);
}
