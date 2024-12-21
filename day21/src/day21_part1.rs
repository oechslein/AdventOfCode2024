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

////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NumericKeyPadState {
    pos: UCoor2D,
    pressed: Vec<char>,
    goal_str: String,
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
    fn new(pos: UCoor2D, goal_str: String) -> Self {
        Self {
            pos,
            pressed: Vec::new(),
            goal_str,
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
        if !self.goal_str.starts_with(&self.get_pressed_key()) {
            return vec![];
        }
        if self.pressed.len() == 4 {
            return vec![];
        }

        let key = *numeric_keypad_grid.get_unchecked(self.pos.x, self.pos.y);
        let press_possible = self.goal_str.chars().nth(self.pressed.len()).unwrap_or(' ') == key;

        if press_possible {
            return vec![Action::Press];
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
    last_action: Option<Action>,
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
            last_action: None,
        }
    }
}

impl<T: KeyPadState> KeyPadState for DirectionalKeyPadState<T> {
    fn execute_action(&self, action: Action, directional_keypad_grid: &GridArray<char>) -> Self {
        debug_assert!(self
            .get_valid_actions(directional_keypad_grid)
            .contains(&action));
        let mut new_state = self.clone();
        new_state.last_action = Some(action.clone());
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
        let inner_pressed_key = self.inner_state.get_pressed_key();
        let inner_pressed_key_len = inner_pressed_key.len();
        if inner_pressed_key
            .rfind('A')
            .unwrap_or(inner_pressed_key_len)
            > 4
        {
            return vec![Action::Press];
        }

        let inner_state_actions = self.inner_state.get_valid_actions(&self.inner_keypad_grid);
        if inner_state_actions.is_empty() {
            return vec![];
        }

        let mut actions = directional_keypad_grid
            .neighborhood_cells(self.pos.x, self.pos.y)
            .filter(|(_coor, &ch)| "<^v>A".contains(ch))
            .map(|(coor, _ch)| Action::Move(self.pos.direction(&coor).unwrap()))
            .collect_vec();

        let key = *directional_keypad_grid.get_unchecked(self.pos.x, self.pos.y);
        let press_possible =
            inner_state_actions
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

////////////////////////////////////////////////////////////////////////////////////

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let result: usize = input
        .lines()
        .par_bridge()
        .map(|goal| {
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

            let numerical_keypad_state =
                NumericKeyPadState::new(UCoor2D::new(2, 3), goal.to_string());
            let inner_directional_keypad_state = DirectionalKeyPadState::new(
                UCoor2D::new(2, 0),
                numerical_keypad_state,
                numerical_keypad_grid,
            );
            let directional_keypad_state = DirectionalKeyPadState::new(
                UCoor2D::new(2, 0),
                inner_directional_keypad_state,
                directional_keypad_grid.clone(),
            );

            let (numeric_part, min_length) =
                solve(&directional_keypad_grid, &directional_keypad_state, goal);

            min_length * numeric_part
        })
        .sum();

    Ok(result.to_string())
}

fn solve(
    directional_keypad_grid: &GridArray<char>,
    directional_keypad_state: &DirectionalKeyPadState<DirectionalKeyPadState<NumericKeyPadState>>,
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

    let result = result.unwrap();

    let action_str2 = result
        .0
        .iter()
        .filter(|state| state.last_action == Some(Action::Press))
        .map(|state| directional_keypad_grid.get_unchecked(state.pos.x, state.pos.y))
        .join("");

    let action_str = result
        .0
        .into_iter()
        .filter_map(|state| state.last_action)
        .join("");
    // println!("{}: {} ---- {}", goal, action_str, action_str2);

    let numeric_part: usize = goal
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .join("")
        .parse()
        .unwrap();

    let min_length = result.1;
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
        let input = include_str!("../input1.txt");
        assert_eq!("184180", process(&input.replace('\r', ""))?);
        Ok(())
    }
}
