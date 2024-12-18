use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;



use std::collections::HashMap;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (rules, updates) = parse_input(input);

    let result: i32 = updates
        .into_par_iter()
        .filter_map(|update| {
            let sorted_update = sorted_by_rules(&update, &rules);
            if update != sorted_update {
                Some(sorted_update[sorted_update.len() / 2])
            } else {
                None
            }
        })
        .sum();

    Ok(result.to_string())
}

fn sorted_by_rules(update: &[i32], rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut sorted_update = update.to_vec();
    sorted_update.sort_unstable_by(|&a, &b| {
        if rules.get(&b).map_or(false, |v| v.contains(&a)) {
            std::cmp::Ordering::Less
        } else if rules.get(&a).map_or(false, |v| v.contains(&b)) {
            std::cmp::Ordering::Greater
        } else {
            a.cmp(&b)
        }
    });
    sorted_update
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let input = input.replace("\r\n", "\n");
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::default();
    for rule in rules_str.lines() {
        let (rule_0_str, rule_1_str) = rule.split_once('|').unwrap();
        let rule_0 = rule_0_str.parse::<i32>().unwrap();
        let rule_1 = rule_1_str.parse::<i32>().unwrap();
        rules.entry(rule_1).or_default().push(rule_0);
    }

    let updates = updates_str
        .lines()
        .map(|update| {
            update
                .split(',')
                .map(|page_str| page_str.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("5331", process(input)?);
        Ok(())
    }
}
