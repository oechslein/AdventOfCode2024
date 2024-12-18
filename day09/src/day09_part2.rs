//use itertools::Itertools;



struct Space {
    id: Option<usize>,
    occupied: usize,
    free: usize,
}

pub fn process(input: &str) -> miette::Result<String> {
    let mut space_list = parse_input(input);
    let highest_id = space_list.iter().filter_map(|s| s.id).max().unwrap_or(0);

    for current_id in (0..=highest_id).rev() {
        compact_space(&mut space_list, current_id);
    }

    let checksum = calculate_checksum(&space_list);

    Ok(checksum.to_string())
}

fn compact_space(space_list: &mut Vec<Space>, current_id: usize) {
    if let Some(current_id_index) = space_list
        .iter()
        .rposition(|space| space.id == Some(current_id))
    {
        let current_id_occupied = space_list[current_id_index].occupied;
        if let Some(free_space_index) = space_list[..current_id_index]
            .iter()
            .position(|space| space.free >= current_id_occupied)
        {
            perform_compaction(space_list, current_id_index, free_space_index);
            //space_list.retain(|space| space.occupied > 0 || space.free > 0);
        }
    }
}

fn perform_compaction(
    space_list: &mut Vec<Space>,
    current_id_index: usize,
    free_space_index: usize,
) {
    let current_id_occupied = space_list[current_id_index].occupied;
    let free_space_free = space_list[free_space_index].free;
    let remaining_free = free_space_free - current_id_occupied;

    space_list[free_space_index].id = space_list[current_id_index].id;
    space_list[free_space_index].occupied = current_id_occupied;
    space_list[free_space_index].free = 0;

    space_list[current_id_index].id = None;
    space_list[current_id_index].occupied = 0;
    space_list[current_id_index].free += current_id_occupied;

    if remaining_free > 0 {
        let new_free_space = Space {
            id: None,
            occupied: 0,
            free: remaining_free,
        };
        space_list.insert(free_space_index + 1, new_free_space);
    }
}

fn calculate_checksum(space_list: &[Space]) -> usize {
    let mut checksum = 0;
    let mut index = 0;
    for space in space_list {
        if let Some(id) = space.id {
            for _ in 0..space.occupied {
                checksum += id * index;
                index += 1;
            }
        }
        index += space.free;
    }
    checksum
}

fn parse_input(input: &str) -> Vec<Space> {
    input
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let length = ch.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                Space {
                    id: Some(i / 2),
                    occupied: length,
                    free: 0,
                }
            } else {
                Space {
                    id: None,
                    occupied: 0,
                    free: length,
                }
            }
        })
        .collect()
}

#[allow(dead_code)]
fn print_space_list(space_list: &Vec<Space>) {
    for space in space_list {
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
        assert_eq!("6287317016845", process(input)?);
        Ok(())
    }
}
