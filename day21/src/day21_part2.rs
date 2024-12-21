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

impl Action {
    fn to_char(&self) -> char {
        match self {
            Action::Move(dir) if *dir == Direction::North => '^',
            Action::Move(dir) if *dir == Direction::South => 'v',
            Action::Move(dir) if *dir == Direction::West => '<',
            Action::Move(dir) if *dir == Direction::East => '>',
            Action::Press => 'A',
            Action::Move(dir) => panic!("Invalid move action: {dir:?}"),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
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

////////////////////////////////////////////////////////////////////////////////////

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
        let press_needed = self.goal_str.chars().nth(self.pressed.len()).unwrap_or(' ') == key;

        if press_needed {
            return vec![Action::Press];
        }

        numeric_keypad_grid
            .neighborhood_cells(self.pos.x, self.pos.y)
            .filter(|(_coor, &ch)| ch != ' ')
            .map(|(coor, _ch)| Action::Move(self.pos.direction(&coor).unwrap()))
            .collect_vec()
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
            > 10
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DummyDirectionalKeyPadState {
    pos: UCoor2D,
    goal_str: String,
    pressed: Vec<char>,
}

impl Display for DummyDirectionalKeyPadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.pos, self.goal_str)
    }
}

impl DummyDirectionalKeyPadState {
    fn new(pos: UCoor2D, goal_str: String) -> Self {
        Self {
            pos,
            goal_str,
            pressed: Vec::new(),
        }
    }
}

impl KeyPadState for DummyDirectionalKeyPadState {
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
                let key = *directional_keypad_grid.get_unchecked(self.pos.x, self.pos.y);
                new_state.pressed.push(key);
            }
            _ => panic!("Invalid action: {:?}", action),
        }
        new_state
    }

    fn get_pressed_key(&self) -> String {
        self.pressed.iter().collect()
    }

    fn get_valid_actions(&self, directional_keypad_grid: &GridArray<char>) -> Vec<Action> {
        if !self.goal_str.starts_with(&self.get_pressed_key()) {
            return vec![];
        }

        let key = *directional_keypad_grid.get_unchecked(self.pos.x, self.pos.y);
        if let Some(next_needed_char) = self.goal_str.chars().nth(self.pressed.len()) {
            if next_needed_char == key {
                return vec![Action::Press];
            }
        }
        directional_keypad_grid
            .neighborhood_cells(self.pos.x, self.pos.y)
            .filter(|(_coor, &ch)| "<^v>A".contains(ch))
            .map(|(coor, _ch)| Action::Move(self.pos.direction(&coor).unwrap()))
            .collect_vec()
    }
}

////////////////////////////////////////////////////////////////////////////////////

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

    let goal = "029A";

    let numerical_keypad_state = NumericKeyPadState::new(UCoor2D::new(2, 3), goal.to_string());
    let directional_keypad_state_1 = DirectionalKeyPadState::new(
        UCoor2D::new(2, 0),
        numerical_keypad_state,
        numerical_keypad_grid,
    );

    ///////////////////////////

    println!("directional_keypad_state_1: {directional_keypad_state_1}");
    let actions = solve_actions(&directional_keypad_grid, &directional_keypad_state_1, goal);
    let action_str = actions.iter().join("");
    println!("action_str: {action_str}");

    /////////////////////////////////////////////

    let easy_cache: FxHashMap<char, String> = "<^>vA"
        .chars()
        .map(|action_char| {
            let (
                result,
                _new_final_directional_keypad_state_pos,
                _new_dummy_directional_keypad_state_pos,
            ) = solve3(
                &directional_keypad_grid,
                &UCoor2D::new(2, 0),
                &UCoor2D::new(2, 0),
                action_char,
            );

            (action_char, result)
        })
        .collect();

    let mut dummy_directional_keypad_state_pos = UCoor2D::new(2, 0);
    let mut final_directional_keypad_state_pos = UCoor2D::new(2, 0);

    let mut cache: FxHashMap<(char, UCoor2D, UCoor2D), String> = FxHashMap::default();
    let mut complete_result = Vec::new();
    for action_char in action_str.chars() {
        let key = (
            action_char,
            final_directional_keypad_state_pos.clone(),
            dummy_directional_keypad_state_pos.clone(),
        );

        let result = if let Some(result) = cache.get(&key) {
            result.clone()
        } else {
            let (
                result,
                new_final_directional_keypad_state_pos,
                new_dummy_directional_keypad_state_pos,
            ) = solve3(
                &directional_keypad_grid,
                &dummy_directional_keypad_state_pos,
                &final_directional_keypad_state_pos,
                action_char,
            );
            dummy_directional_keypad_state_pos = new_dummy_directional_keypad_state_pos;
            final_directional_keypad_state_pos = new_final_directional_keypad_state_pos;
            result
        };

        println!(
            "[({},{},{}) => {}",
            final_directional_keypad_state_pos,
            dummy_directional_keypad_state_pos,
            action_char,
            result
        );

        assert!(easy_cache.get(&action_char).unwrap() == &result);
        complete_result.extend(result.chars());
        cache.insert(key, result);
    }

    println!("complete_result 1: {}", complete_result.iter().join(""));

    ///////////////////////////////////////////////////////////////////////////////////

    let action_str = complete_result.into_iter().join("");

    let mut dummy_directional_keypad_state_pos = UCoor2D::new(2, 0);
    let mut final_directional_keypad_state_pos = UCoor2D::new(2, 0);

    let mut new_cache: FxHashMap<(char, UCoor2D, UCoor2D), String> = FxHashMap::default();
    let mut complete_result = Vec::new();
    for action_char in action_str.chars() {
        let key = (
            action_char,
            final_directional_keypad_state_pos.clone(),
            dummy_directional_keypad_state_pos.clone(),
        );

        let result = if let Some(result) = new_cache.get(&key) {
            assert_eq!(result, cache.get(&key).unwrap());
            result.clone()
        } else {
            let (
                result,
                new_final_directional_keypad_state_pos,
                new_dummy_directional_keypad_state_pos,
            ) = solve3(
                &directional_keypad_grid,
                &dummy_directional_keypad_state_pos,
                &final_directional_keypad_state_pos,
                action_char,
            );
            dummy_directional_keypad_state_pos = new_dummy_directional_keypad_state_pos;
            final_directional_keypad_state_pos = new_final_directional_keypad_state_pos;
            result
        };

        println!(
            "[({},{},{}) => {}",
            final_directional_keypad_state_pos,
            dummy_directional_keypad_state_pos,
            action_char,
            result
        );

        complete_result.extend(result.chars());
        new_cache.insert(key, result);
    }

    println!("complete_result: {}", complete_result.iter().join(""));

    Ok(42.to_string())
}

fn solve3(
    directional_keypad_grid: &GridArray<char>,
    dummy_directional_keypad_state_pos: &UCoor2D,
    final_directional_keypad_state_pos: &UCoor2D,
    action_char: char,
) -> (String, UCoor2D, UCoor2D) {
    let action_str = action_char.to_string();

    let dummy_directional_keypad_state = DummyDirectionalKeyPadState::new(
        dummy_directional_keypad_state_pos.clone(),
        action_str.clone(),
    );
    let final_directional_keypad_state = DirectionalKeyPadState::new(
        final_directional_keypad_state_pos.clone(),
        dummy_directional_keypad_state,
        directional_keypad_grid.clone(),
    );
    let result = dijkstra(
        &final_directional_keypad_state,
        |directional_keypad_state| {
            directional_keypad_state
                .get_valid_actions(directional_keypad_grid)
                .into_iter()
                .map(|action| {
                    (
                        directional_keypad_state.execute_action(action, directional_keypad_grid),
                        1,
                    )
                })
                .collect_vec()
        },
        |directional_keypad_state| directional_keypad_state.get_pressed_key() == action_str,
    );

    let result = result
        .unwrap()
        .0
        .into_iter()
        .filter_map(|state| state.last_action)
        .join("");

    let final_directional_keypad_state_pos = final_directional_keypad_state.pos;
    let final_directional_keypad_state_inner_state_pos =
        final_directional_keypad_state.inner_state.pos;
    (
        result,
        final_directional_keypad_state_pos,
        final_directional_keypad_state_inner_state_pos,
    )
}

fn solve_actions<T: KeyPadState + Eq + Hash>(
    directional_keypad_grid: &GridArray<char>,
    directional_keypad_state: &DirectionalKeyPadState<T>,
    goal: &str,
) -> Vec<Action> {
    let directional_keypad_state = directional_keypad_state.clone();
    let result = dijkstra(
        &directional_keypad_state,
        |directional_keypad_state| {
            directional_keypad_state
                .get_valid_actions(directional_keypad_grid)
                .into_iter()
                .map(|action| {
                    (
                        directional_keypad_state.execute_action(action, directional_keypad_grid),
                        1,
                    )
                })
                .collect_vec()
        },
        |directional_keypad_state| directional_keypad_state.get_pressed_key() == *goal,
    );

    result
        .unwrap()
        .0
        .into_iter()
        .filter_map(|state| state.last_action)
        .collect_vec()
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
