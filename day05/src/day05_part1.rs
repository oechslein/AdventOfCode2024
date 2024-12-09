use fxhash::FxHashMap;
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;
use smallvec::SmallVec;

use crate::custom_error::AocError;

type Page = u8;
type PageVec = SmallVec<[Page; 20]>;

fn parse_page(rule_0_str: &str) -> Page {
    rule_0_str.parse::<Page>().unwrap()
}

fn parse_rule(rule: &str) -> (u8, u8) {
    let (rule_0_str, rule_1_str) = rule.split_once('|').unwrap();
    (parse_page(rule_0_str), parse_page(rule_1_str))
}

fn parse_page_vec(update: &str) -> PageVec {
    update.split(',').map(parse_page).collect::<PageVec>()
}

fn parse_rules(rules_str: &str) -> FxHashMap<Page, PageVec> {
    let mut rules: FxHashMap<Page, PageVec> = FxHashMap::default();
    for rule in rules_str.lines() {
        let (rule_0, rule_1) = parse_rule(rule);
        rules.entry(rule_1).or_default().push(rule_0);
    }
    rules
}

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input = input.replace("\r\n", "\n");
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);
    let updates = updates_str.lines().map(parse_page_vec);

    let result: usize = updates
        .filter(|update| is_sorted_by_rules(update, &rules))
        .map(|sorted_update| sorted_update[sorted_update.len() / 2] as usize)
        .sum();

    Ok(result.to_string())
}

fn is_sorted_by_rules(update: &[Page], ordering_rules: &FxHashMap<Page, PageVec>) -> bool {
    update.windows(2).all(|window| {
        let a = &window[0];
        let b = &window[1];

        let b_should_be_greater_than_a = || {
            ordering_rules
                .get(b)
                .map_or(true, |b_rules| b_rules.contains(a))
        };

        let a_should_not_be_greater_than_b = || {
            ordering_rules
                .get(a)
                .map_or(true, |a_rules| !a_rules.contains(b))
        };

        b_should_be_greater_than_a() && a_should_not_be_greater_than_b()
    })
}

fn is_sorted_by_rules_2(update: &[Page], ordering_rules: &FxHashMap<Page, PageVec>) -> bool {
    // Iterate over pairs of adjacent elements
    for window in update.windows(2) {
        let a = window[0];
        let b = window[1];

        // Check if there are specific ordering rules between a and b
        if let Some(a_rules) = ordering_rules.get(&a) {
            if a_rules.contains(&b) {
                return false; // a should not come before b
            }
        }

        if let Some(b_rules) = ordering_rules.get(&b) {
            if !b_rules.contains(&a) {
                return false; // b should not come before a
            }
        }
    }
    true // All pairs are valid according to the rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
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
        assert_eq!("143", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("5588", process(input)?);
        Ok(())
    }
}
