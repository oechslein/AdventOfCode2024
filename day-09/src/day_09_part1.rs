use itertools::Itertools;

use crate::custom_error::AocError;

struct Space {
    id: Option<usize>,
    occupied: usize,
    free: usize,
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut space_list = vec![];
    let mut index = 0;
    let mut free_space = false;
    let mut id = 0;
    for ch in input.chars() {
        let length = ch.to_digit(10).unwrap() as usize;
        space_list.push(if free_space {
            Space {
                id: None,
                occupied: 0,
                free: length,
            }
        } else {
            Space {
                id: Some(id),
                occupied: length,
                free: 0,
            }
        });
        free_space = !free_space;
        index = index + length;
        if !free_space {
            id += 1
        }
    }

    //print_space_list(&space_list);

    let mut curr_free_index = space_list
        .iter()
        .enumerate()
        .filter(|(_, space)| space.free > 0)
        .map(|(index, _)| index)
        .next()
        .unwrap();
    let mut last_occupied_index = space_list
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, space)| space.occupied > 0)
        .map(|(index, _)| index)
        .next()
        .unwrap();

    while curr_free_index < last_occupied_index {
        if space_list[curr_free_index].id.is_none() {
            space_list[curr_free_index].id = space_list[last_occupied_index].id;
        }

        if space_list[last_occupied_index].id == space_list[curr_free_index].id {
            space_list[last_occupied_index].free += 1;
            space_list[last_occupied_index].occupied -= 1;
            space_list[curr_free_index].free -= 1;
            space_list[curr_free_index].occupied += 1;

        } else if space_list[curr_free_index].free > 0 {
            // split free space since id is different
            let new_space = Space {
                id: space_list[last_occupied_index].id,
                free: space_list[curr_free_index].free,
                occupied: 0,
            };
            space_list[curr_free_index].free = 0;
            curr_free_index += 1;
            space_list.insert(curr_free_index, new_space);
            last_occupied_index += 1;
        }

        if space_list[curr_free_index].free == 0 {
            curr_free_index = space_list
                .iter()
                .enumerate()
                .filter(|(_, space)| space.free > 0)
                .map(|(index, _)| index)
                .next()
                .unwrap();
        }
        if space_list[last_occupied_index].occupied == 0 {
            last_occupied_index = space_list
                .iter()
                .enumerate()
                .rev()
                .filter(|(_, space)| space.occupied > 0)
                .map(|(index, _)| index)
                .next()
                .unwrap();
        }

        //print_space_list(&space_list);
    }

    let mut checksum = 0;
    let mut index = 0;
    for space in space_list {
        for _ in 0..space.occupied {
            checksum += space.id.unwrap() * index;
            index += 1;
        }
        index += space.free;
    }

    Ok(checksum.to_string())
}

fn print_space_list(space_list: &Vec<Space>) {
    for space in space_list.iter() {
        for _index in 0..space.occupied {
            print!("{}", space.id.unwrap());
        }
        for _index in 0..space.free {
            print!(".");
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!(1928.to_string(), process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!(6262891638328.to_string(), process(input)?);
        Ok(())
    }
}
