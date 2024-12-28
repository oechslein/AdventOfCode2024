use std::{
    fmt::Display,
    mem::swap,
    str::FromStr,
    sync::{LazyLock, RwLock},
};

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub(crate) struct Puzzle {
    pub(crate) wire_values: FxHashMap<Wire, bool>,
    pub(crate) ops: FxHashMap<Wire, Expr>,
}

impl Puzzle {
    pub(crate) fn all_wires(&self) -> impl Iterator<Item = Wire> + use<'_> {
        self.ops.keys().cloned()
    }

    pub(crate) fn inputs_count(&self) -> u8 {
        self.wire_values
            .keys()
            .filter(|wire| matches!(wire, Wire::InputX(_)))
            .count() as u8
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum Wire {
    InputX(u8),
    InputY(u8),
    OutputZ(u8),
    Other(u8),
}

impl PartialOrd for Wire {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Wire {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Wire::InputX(a), Wire::InputX(b))
            | (Wire::InputY(a), Wire::InputY(b))
            | (Wire::OutputZ(a), Wire::OutputZ(b))
            | (Wire::Other(a), Wire::Other(b)) => a.cmp(b),

            (Wire::InputY(_), Wire::InputX(_))
            | (Wire::OutputZ(_), Wire::InputY(_))
            | (Wire::Other(_), Wire::OutputZ(_)) => std::cmp::Ordering::Greater,

            (Wire::InputX(_) | Wire::InputY(_) | Wire::OutputZ(_) | Wire::Other(_), _) => {
                std::cmp::Ordering::Less
            }
        }
    }
}

static INTERIM_WIRE_NAME_MAP: LazyLock<RwLock<FxHashMap<String, u8>>> =
    LazyLock::new(|| RwLock::new(FxHashMap::default()));

impl FromStr for Wire {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(remaining_str) = s.strip_prefix('x') {
            Ok(Wire::InputX(remaining_str.parse().unwrap()))
        } else if let Some(remaining_str) = s.strip_prefix('y') {
            Ok(Wire::InputY(remaining_str.parse().unwrap()))
        } else if let Some(remaining_str) = s.strip_prefix('z') {
            Ok(Wire::OutputZ(remaining_str.parse().unwrap()))
        } else {
            let id: u8 = {
                let key = s.to_string();
                let mut map = INTERIM_WIRE_NAME_MAP.write().unwrap();
                if let Some(id) = map.get(&key) {
                    *id
                } else {
                    let id = map.values().max().unwrap_or(&0) + 1;
                    map.insert(key, id);
                    id
                }
            };
            Ok(Wire::Other(id))
        }
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Wire::InputX(num) => format!("x{num:02}"),
            Wire::InputY(num) => format!("y{num:02}"),
            Wire::OutputZ(num) => format!("z{num:02}"),
            Wire::Other(num) => {
                let map = INTERIM_WIRE_NAME_MAP.read().unwrap();
                map.iter()
                    .find(|(_name, id)| id == &num)
                    .unwrap()
                    .0
                    .to_string()
            }
        })
    }
}

#[allow(dead_code)]
impl Wire {
    pub(crate) fn value_of(&self, puzzle: &Puzzle) -> Option<bool> {
        self.value_of_inner(puzzle, 0)
    }

    fn value_of_inner(&self, puzzle: &Puzzle, depth: usize) -> Option<bool> {
        if let Some(value) = puzzle.wire_values.get(self) {
            Some(*value)
        } else {
            debug_assert!(!matches!(self, Wire::InputX(_) | Wire::InputY(_)));
            puzzle.ops[self].execute_inner(puzzle, depth + 1)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    pub(crate) fn apply(self, lhs_value: bool, rhs_value: bool) -> bool {
        match self {
            Op::And => lhs_value & rhs_value,
            Op::Or => lhs_value | rhs_value,
            Op::Xor => lhs_value ^ rhs_value,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Expr {
    pub(crate) op: Op,
    pub(crate) lhs: Wire,
    pub(crate) rhs: Wire,
}

#[allow(dead_code)]
impl Expr {
    pub(crate) fn execute(&self, puzzle: &Puzzle) -> Option<bool> {
        self.execute_inner(puzzle, 0)
    }

    fn execute_inner(&self, puzzle: &Puzzle, depth: usize) -> Option<bool> {
        if depth > puzzle.ops.len() {
            return None;
        }
        if let Some(lhs_value) = self.lhs.value_of_inner(puzzle, depth) {
            if let Some(rhs_value) = self.rhs.value_of_inner(puzzle, depth) {
                return Some(self.op.apply(lhs_value, rhs_value));
            }
        }
        None
    }
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let (initial, connections) = input.split_once("\n\n").unwrap();
        let wire_values = initial
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(": ").unwrap();
                (Wire::from_str(left).unwrap(), right == "1")
            })
            .collect();
        let ops = connections
            .lines()
            .map(|line| {
                let (input, output) = line.split_once(" -> ").unwrap();
                let (lhs_str, op_str, rhs_str) = input.split_whitespace().collect_tuple().unwrap();
                let op = match op_str {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "XOR" => Op::Xor,
                    _ => panic!("at the disco"),
                };

                let mut lhs = Wire::from_str(lhs_str).unwrap();
                let mut rhs = Wire::from_str(rhs_str).unwrap();
                if lhs > rhs {
                    swap(&mut lhs, &mut rhs);
                }
                (Wire::from_str(output).unwrap(), Expr { op, lhs, rhs })
            })
            .collect();
        Ok(Puzzle { wire_values, ops })
    }
}
