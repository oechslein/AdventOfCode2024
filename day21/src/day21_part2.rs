use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_types::{Direction, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;

use std::{
    fmt::{Debug as FmtDebug, Display},
    hash::Hash,
    str::FromStr,
};

use miette::{miette, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Action {
    Move(Direction),
    Press,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Move(dir) if *dir == Direction::North => "^",
                Action::Move(dir) if *dir == Direction::South => "v",
                Action::Move(dir) if *dir == Direction::West => "<",
                Action::Move(dir) if *dir == Direction::East => ">",
                Action::Press => "A",
                Action::Move(dir) => panic!("Invalid move action: {dir:?}"),
            }
        )
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "^" => Action::Move(Direction::North),
            "v" => Action::Move(Direction::South),
            "<" => Action::Move(Direction::West),
            ">" => Action::Move(Direction::East),
            "A" => Action::Press,
            _ => panic!("Invalid action: {}", s),
        })
    }
}

trait KeyPadState: Display + FmtDebug + Clone + PartialEq {
    fn execute_action(&self, action: Action, keypad_grid: &GridArray<char>) -> Self;
    fn get_pressed_key(&self) -> String;
    fn get_valid_actions(&self, keypad_grid: &GridArray<char>) -> Vec<Action>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NumericKeyPadState {
    pos: UCoor2D,
    pressed: Vec<char>,
}

impl Display for NumericKeyPadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/'{}'",
            self.pos,
            self.pressed.iter().collect::<String>()
        )
    }
}

impl NumericKeyPadState {
    fn new(pos: UCoor2D) -> Self {
        Self {
            pos,
            pressed: Vec::new(),
        }
    }
}

impl KeyPadState for NumericKeyPadState {
    fn execute_action(&self, action: Action, numeric_keypad_grid: &GridArray<char>) -> Self {
        debug_assert!(self
            .get_valid_actions(numeric_keypad_grid)
            .contains(&action));
        let mut new_state = self.clone();
        match action {
            Action::Move(Direction::North) => new_state.pos.y -= 1,
            Action::Move(Direction::South) => new_state.pos.y += 1,
            Action::Move(Direction::West) => new_state.pos.x -= 1,
            Action::Move(Direction::East) => new_state.pos.x += 1,
            Action::Press => {
                let key = numeric_keypad_grid.get_unchecked(new_state.pos.x, new_state.pos.y);
                debug_assert_ne!(*key, ' ');
                new_state.pressed.push(*key);
            }
            _ => panic!("Invalid action: {:?}", action),
        }
        new_state
    }

    fn get_pressed_key(&self) -> String {
        self.pressed.iter().collect()
    }

    fn get_valid_actions(&self, numeric_keypad_grid: &GridArray<char>) -> Vec<Action> {
        if self.pressed.len() == 4 {
            return vec![];
        }
        let mut actions = numeric_keypad_grid
            .neighborhood_cells(self.pos.x, self.pos.y)
            .filter(|(_coor, &ch)| ch != ' ')
            .map(|(coor, _ch)| Action::Move(self.pos.direction(&coor).unwrap()))
            .collect_vec();

        actions.push(Action::Press);
        actions
    }
}

////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DirectionalKeyPadState<T: KeyPadState> {
    pos: UCoor2D,
    inner_state: T,
    inner_keypad_grid: GridArray<char>,
}

impl<T: KeyPadState> Display for DirectionalKeyPadState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.pos, self.inner_state)
    }
}

impl<T: KeyPadState> DirectionalKeyPadState<T> {
    fn new(pos: UCoor2D, inner_state: T, inner_keypad_grid: GridArray<char>) -> Self {
        Self {
            pos,
            inner_state,
            inner_keypad_grid,
        }
    }
}

impl<T: KeyPadState> KeyPadState for DirectionalKeyPadState<T> {
    fn execute_action(&self, action: Action, directional_keypad_grid: &GridArray<char>) -> Self {
        debug_assert!(self
            .get_valid_actions(directional_keypad_grid)
            .contains(&action));
        let mut new_state = self.clone();
        match action {
            Action::Move(Direction::North) => new_state.pos.y -= 1,
            Action::Move(Direction::South) => new_state.pos.y += 1,
            Action::Move(Direction::West) => new_state.pos.x -= 1,
            Action::Move(Direction::East) => new_state.pos.x += 1,
            Action::Press => {
                let key = directional_keypad_grid.get_unchecked(new_state.pos.x, new_state.pos.y);
                new_state.inner_state = match key {
                    '^' => new_state
                        .inner_state
                        .execute_action(Action::Move(Direction::North), &self.inner_keypad_grid),
                    'v' => new_state
                        .inner_state
                        .execute_action(Action::Move(Direction::South), &self.inner_keypad_grid),
                    '<' => new_state
                        .inner_state
                        .execute_action(Action::Move(Direction::West), &self.inner_keypad_grid),
                    '>' => new_state
                        .inner_state
                        .execute_action(Action::Move(Direction::East), &self.inner_keypad_grid),
                    'A' => new_state
                        .inner_state
                        .execute_action(Action::Press, &self.inner_keypad_grid),
                    _ => panic!("Invalid key: {}", key),
                };
            }
            _ => panic!("Invalid action: {:?}", action),
        }
        new_state
    }

    fn get_pressed_key(&self) -> String {
        self.inner_state.get_pressed_key()
    }

    fn get_valid_actions(&self, directional_keypad_grid: &GridArray<char>) -> Vec<Action> {
        let mut actions = directional_keypad_grid
            .neighborhood_cells(self.pos.x, self.pos.y)
            .filter(|(_coor, &ch)| "<^v>A".contains(ch))
            .map(|(coor, _ch)| Action::Move(self.pos.direction(&coor).unwrap()))
            .collect_vec();

        let key = *directional_keypad_grid.get_unchecked(self.pos.x, self.pos.y);
        let press_possible = self
            .inner_state
            .get_valid_actions(&self.inner_keypad_grid)
            .into_iter()
            .any(|inner_action| match inner_action {
                Action::Move(dir) if dir == Direction::North && key == '^' => true,
                Action::Move(dir) if dir == Direction::South && key == 'v' => true,
                Action::Move(dir) if dir == Direction::West && key == '<' => true,
                Action::Move(dir) if dir == Direction::East && key == '>' => true,
                Action::Press if key == 'A' => true,
                _ => false,
            });
        if press_possible {
            actions.push(Action::Press);
        }
        actions
    }
}

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let numerical_keypad_grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        "789\n456\n123\n 0A",
    );
    let directional_keypad_grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        " ^A\n<v>",
    );
    ///////////////////////////

    let numerical_keypad_state = NumericKeyPadState::new(UCoor2D::new(2, 3));
    let directional_keypad_state_1 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        numerical_keypad_state,
        numerical_keypad_grid,
    );
    let directional_keypad_state_2 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_1,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_3 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_2,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_4 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_3,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_5 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_4,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_6 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_5,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_7 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_6,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_8 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_7,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_9 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_8,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_10 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_9,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_11 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_10,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_12 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_11,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_13 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_12,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_14 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_13,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_15 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_14,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_16 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_15,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_17 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_16,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_18 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_17,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_19 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_18,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_20 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_19,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_21 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_20,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_22 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_21,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_23 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_22,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_24 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_23,
        directional_keypad_grid.clone(),
    );
    let directional_keypad_state_25 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_24,
        directional_keypad_grid.clone(),
    );

    let final_directional_keypad_state = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        directional_keypad_state_25,
        directional_keypad_grid.clone(),
    );

    // let moves = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
    // for curr_move in moves.chars() {
    //     let action = Action::from_str(&curr_move.to_string())?;
    //     directional_keypad_state =
    //         directional_keypad_state.execute_action(action, &directional_keypad_grid);
    // }

    // println!(
    //     "{directional_keypad_state}\n{:?}",
    //     directional_keypad_state.get_valid_actions(&directional_keypad_grid)
    // );

    let result: usize = input
        .lines()
        .par_bridge()
        .map(|goal| {
            let (numeric_part, min_length) = solve(
                &directional_keypad_grid,
                &final_directional_keypad_state,
                goal,
            );
            println!("{}: {}*{}", goal, min_length, numeric_part);

            min_length * numeric_part
        })
        .sum();

    Ok(result.to_string())
}

fn solve<T: KeyPadState + Eq + Hash>(
    directional_keypad_grid: &GridArray<char>,
    directional_keypad_state: &T,
    goal: &str,
) -> (usize, usize) {
    let directional_keypad_state = directional_keypad_state.clone();
    let result = dijkstra(
        &directional_keypad_state,
        |directional_keypad_state| {
            directional_keypad_state
                .get_valid_actions(directional_keypad_grid)
                .into_iter()
                .map(|action| {
                    let new_state =
                        directional_keypad_state.execute_action(action, directional_keypad_grid);
                    (new_state, 1)
                })
                .collect_vec()
        },
        |directional_keypad_state| directional_keypad_state.get_pressed_key() == *goal,
    );

    let numeric_part: usize = goal
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .join("")
        .parse()
        .unwrap_or(0);
    let min_length = result.unwrap().1;
    (numeric_part, min_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("", process(&input.replace('\r', ""))?);
        Ok(())
    }
}
