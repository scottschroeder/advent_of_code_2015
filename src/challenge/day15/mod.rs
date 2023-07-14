use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::challenge::day15::permute_distribute::PermuteDistribute;

const PROPERTY_LEN: usize = 4;
const SCORED_PROPERTIES: [&str; PROPERTY_LEN] = ["capacity", "durability", "flavor", "texture"];

mod permute_distribute;

pub fn part1(input: &str) -> Result<String> {
    let x = 0;
    let ingredients = parse_ingredient_list(input)?;
    log::debug!("{:#?}", ingredients);

    let mut d = PermuteDistribute::new(2, 10);
    log::debug!("{:#?}", d);
    d.pop();
    log::debug!("{:#?}", d);


    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let x = 0;
    Ok(format!("{:?}", x))
}

#[derive(Debug)]
struct Ingredient<'a> {
    name: &'a str,
    properties: [i64; PROPERTY_LEN],
    calories: i64,
}

fn score_ingredients(ingredients: &[Ingredient], quantities: &[usize]) -> i64 {
    let mut scores = [0; PROPERTY_LEN];
    for (q, ingredient) in quantities.iter().zip(ingredients) {
        for (score, prop_value) in scores.iter_mut().zip(ingredient.properties) {
            *score += prop_value * (*q as i64);
        }
    }
    for s in scores {
        if s <= 0 {
            return 0;
        }
    }
    scores.iter().fold(1i64, |acc, x| acc * (*x))
}

fn parse_property(input: &str) -> Result<(&str, i64)> {
    let (kind, value_input) = input
        .split_once(' ')
        .ok_or_else(|| anyhow::anyhow!("property was not delimited"))?;

    let value = value_input
        .parse::<i64>()
        .context("parse prop value as int")?;

    Ok((kind, value))
}

fn parse_ingredient(input: &str) -> Result<Ingredient> {
    let (name, prop_input) = input
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("ingredient did not have `:` delimiter"))?;

    let properties = prop_input
        .split(',')
        .map(|prop| parse_property(prop.trim()))
        .collect::<Result<HashMap<_, _>>>()?;

    let get_prop = |prop: &str| {
        properties
            .get(prop)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("properties for `{}` did not contain: {:?}", name, prop))
    };

    Ok(Ingredient {
        name,
        properties: [
            get_prop("capacity")?,
            get_prop("durability")?,
            get_prop("flavor")?,
            get_prop("texture")?,
        ],
        calories: get_prop("calories")?,
    })
}

fn parse_ingredient_list(input: &str) -> Result<Vec<Ingredient>> {
    input
        .lines()
        .map(|line| parse_ingredient(line).with_context(|| format!("parse: {:?}", line)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day15");
    const INPUT_EX: &str = include_str!("../../../input/day15_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "0")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "0")
    }

    #[test]
    fn check_score_function() {
        let ingredients = parse_ingredient_list(INPUT_EX).unwrap();
        let score = score_ingredients(&ingredients, &[44, 56]);
        assert_eq!(score, 62842880);
    }
    #[test]
    fn check_score_function_negative() {
        let ingredients = parse_ingredient_list(INPUT_EX).unwrap();
        let score = score_ingredients(&ingredients, &[3, 1]);
        assert_eq!(score, 0);
    }
}
