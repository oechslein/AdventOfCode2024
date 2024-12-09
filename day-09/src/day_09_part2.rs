use itertools::Itertools;

use crate::custom_error::AocError;

struct Space {
    id: Option<usize>,
    occupied: usize,
    free: usize,
    processed: bool,
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut space_list = vec![];
    let mut index = 0;
    let mut free_space = false;
    let mut id = 0;
    let mut highest_id = 0;
    for ch in input.chars() {
        let length = ch.to_digit(10).unwrap() as usize;
        space_list.push(if free_space {
            Space {
                id: None,
                occupied: 0,
                free: length,
                processed: false,
            }
        } else {
            highest_id = highest_id.max(id);
            Space {
                id: Some(id),
                occupied: length,
                free: 0,
                processed: false,
            }
        });
        free_space = !free_space;
        index = index + length;
        if !free_space {
            id += 1
        }
    }

    //print_space_list(&space_list);

    loop {
        let highest_id_index_opt = space_list
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, space)| {
                space.occupied > 0 && space.id == Some(highest_id) && !space.processed
            })
            .map(|(index, _)| index)
            .next();
        match highest_id_index_opt {
            None if highest_id == 0 => {
                break;
            }
            None => {
                highest_id -= 1;
                continue;
            }
            Some(highest_id_index) => {
                let curr_free_fitting_index_opt = space_list
                    .iter()
                    .enumerate()
                    .filter(|(index, space)| {
                        space.free >= space_list[highest_id_index].occupied
                            && *index < highest_id_index
                    })
                    .map(|(index, _)| index)
                    .next();
                match curr_free_fitting_index_opt {
                    None => {
                        space_list[highest_id_index].processed = true;
                        continue;
                    }
                    Some(curr_free_fitting_index) => {
                        space_list[curr_free_fitting_index].id = space_list[highest_id_index].id;
                        space_list[curr_free_fitting_index].occupied =
                            space_list[highest_id_index].occupied;
                        let remaining_space = space_list[curr_free_fitting_index].free
                            - space_list[curr_free_fitting_index].occupied;
                        space_list[curr_free_fitting_index].free = 0;
                        space_list[curr_free_fitting_index].processed = true;

                        space_list[highest_id_index].id = None;
                        space_list[highest_id_index].free += space_list[highest_id_index].occupied;
                        space_list[highest_id_index].occupied = 0;
                        space_list[highest_id_index].processed = true;

                        if remaining_space > 0 {
                            let new_free_space = Space {
                                id: None,
                                free: remaining_space,
                                occupied: 0,
                                processed: false,
                            };
                            space_list.insert(curr_free_fitting_index + 1, new_free_space);
                        }
                    }
                }
            }
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
        for _ in 0..space.free {
            index += 1;
        }
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
    fn test_part2_example() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!(2858.to_string(), process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(6287317016845.to_string(), process(input)?);
        Ok(())
    }
}
