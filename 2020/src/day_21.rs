/*
    --- Day 21: Allergen Assessment ---
    You reach the train's last stop and the closest you can get to your vacation island without getting wet. There aren't even any boats here, but nothing can stop you now: you build a raft. You just need a few days' worth of food for your journey.

    You don't speak the local language, so you can't read any ingredients lists. However, sometimes, allergens are listed in a language you do understand. You should be able to use this information to determine which ingredient contains which allergen and work out which foods are safe to take with you on your trip.

    You start by compiling a list of foods (your puzzle input), one food per line. Each line includes that food's ingredients list followed by some or all of the allergens the food contains.

    Each allergen is found in exactly one ingredient. Each ingredient contains zero or one allergen. Allergens aren't always marked; when they're listed (as in (contains nuts, shellfish) after an ingredients list), the ingredient that contains each listed allergen will be somewhere in the corresponding ingredients list. However, even if an allergen isn't listed, the ingredient that contains that allergen could still be present: maybe they forgot to label it, or maybe it was labeled in a language you don't know.

    For example, consider the following list of foods:

    mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)
    The first food in the list has four ingredients (written in a language you don't understand): mxmxvkd, kfcds, sqjhc, and nhms. While the food might contain other allergens, a few allergens the food definitely contains are listed afterward: dairy and fish.

    The first step is to determine which ingredients can't possibly contain any of the allergens in any food in your list. In the above example, none of the ingredients kfcds, nhms, sbzzf, or trh can contain an allergen. Counting the number of times any of these ingredients appear in any ingredients list produces 5: they all appear once each except sbzzf, which appears twice.

    Determine which ingredients cannot possibly contain any of the allergens in your list. How many times do any of those ingredients appear?

    --- Part Two ---
    Now that you've isolated the inert ingredients, you should have enough information to figure out which ingredient contains which allergen.

    In the above example:

    mxmxvkd contains dairy.
    sqjhc contains fish.
    fvjkl contains soy.
    Arrange the ingredients alphabetically by their allergen and separate them by commas to produce your canonical dangerous ingredient list. (There should not be any spaces in your canonical dangerous ingredient list.) In the above example, this would be mxmxvkd,sqjhc,fvjkl.

    Time to stock your raft with supplies. What is your canonical dangerous ingredient list?
*/

use crate::common::{to_owned, trim_start};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char},
    multi::{many1, separated_list1},
    sequence::{delimited, pair},
    IResult,
};
use std::collections::HashSet;

type Ingredient = String;
type Allergen = String;

#[derive(Clone)]
pub struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

impl Food {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (ingredients, allergens)) = pair(
            trim_start(separated_list1(char(' '), to_owned(alpha1))),
            delimited(
                tag(" (contains "),
                separated_list1(tag(", "), to_owned(alpha1)),
                char(')'),
            ),
        )(input)?;

        Ok((
            input,
            Self {
                ingredients,
                allergens,
            },
        ))
    }
}

#[derive(Clone)]
pub struct FoodList {
    food: Vec<Food>,
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

impl FoodList {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, food) = many1(Food::parser)(input)?;

        let ingredients: HashSet<Ingredient> =
            food.iter().flat_map(|f| f.ingredients.clone()).collect();
        let allergens: HashSet<Allergen> = food.iter().flat_map(|f| f.allergens.clone()).collect();

        Ok((
            input,
            Self {
                food,
                ingredients,
                allergens,
            },
        ))
    }

    fn possible_ingredients(&self, allergen: &str) -> HashSet<Ingredient> {
        // Get all foods that have this allergen listed. Get a set of each food's ingredients.
        let mut ingredients: Vec<HashSet<Ingredient>> = Vec::new();
        for f in &self.food {
            if f.allergens.iter().any(|a| a == allergen) == true {
                let set: HashSet<String> = f.ingredients.iter().cloned().collect();
                ingredients.push(set);
            }
        }

        // Find the intersection of all ingredients contained in foods with the allergen
        let mut iter = ingredients.into_iter();
        let start: HashSet<Ingredient> = iter.next().unwrap();
        iter.fold(start, |acc, set| &acc & &set)
    }

    fn no_allergen_count(&self) -> usize {
        // First, find all ingredients that have no possible allergens
        let mut no_allergens: HashSet<Ingredient> = self.ingredients.clone();
        for a in &self.allergens {
            for i in self.possible_ingredients(&a) {
                no_allergens.remove(&i);
            }
        }

        // Last, count how many times these ingredients appear in all foods
        self.food
            .iter()
            .map(|f| {
                f.ingredients
                    .iter()
                    .filter(|&i| no_allergens.contains(i))
                    .count()
            })
            .sum()
    }

    fn match_ingredients_allergens(&self) -> Vec<(Ingredient, Allergen)> {
        let mut output: Vec<(Ingredient, Allergen)> = Vec::new();
        let mut possible_allergens: Vec<(Allergen, HashSet<Ingredient>)> = self
            .allergens
            .iter()
            .map(|a| (a.clone(), self.possible_ingredients(a)))
            .collect();

        while possible_allergens.is_empty() == false {
            // Assume the input is kind to us, and there is at least one allergen that has only one
            // possible Ingredient. Find one of these ingredients.
            let mut remove_idx: Option<usize> = None;
            for (i, (_allergen, ingredients)) in possible_allergens.iter().enumerate() {
                if ingredients.len() == 1 {
                    remove_idx = Some(i);
                    break;
                }
            }

            // Next remove the allergen from the list of possible allergens, and the ingredient from all allergens.
            let (allergen, ingredients) = possible_allergens.remove(remove_idx.unwrap());
            assert_eq!(ingredients.len(), 1);
            let ingredient = ingredients.iter().next().unwrap().clone();
            for (_a, i) in possible_allergens.iter_mut() {
                i.remove(&ingredient);
            }

            // Last add the ingredient / allergen to the output list
            output.push((ingredient, allergen));
        }

        output
    }

    fn ingredient_string(pairs: &[(Ingredient, Allergen)]) -> String {
        let mut sorted = pairs.to_vec();
        sorted.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        let ingredients: Vec<&str> = sorted.iter().map(|(i, _a)| i as &str).collect();
        ingredients.join(",")
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> FoodList {
    FoodList::parser(input).unwrap().1
}

#[aoc(day21, part1)]
pub fn part1(input: &FoodList) -> usize {
    let count = input.no_allergen_count();
    assert_eq!(count, 2061);
    count
}

#[aoc(day21, part2)]
pub fn part2(input: &FoodList) -> String {
    let pairs = input.match_ingredients_allergens();
    let string = FoodList::ingredient_string(&pairs);
    assert_eq!(string, "cdqvp,dglm,zhqjs,rbpg,xvtrfz,tgmzqjz,mfqgx,rffqhl");
    string
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_no_allergen_count() {
        let food_list = input_generator(EXAMPLE_INPUT);
        let count = food_list.no_allergen_count();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_match_ingredients_allergens() {
        let food_list = input_generator(EXAMPLE_INPUT);
        let pairs = food_list.match_ingredients_allergens();
        assert_eq!(
            pairs,
            [
                ("mxmxvkd".into(), "dairy".into()),
                ("sqjhc".into(), "fish".into()),
                ("fvjkl".into(), "soy".into()),
            ]
        );
    }

    #[test]
    fn test_ingredient_string() {
        let food_list = input_generator(EXAMPLE_INPUT);
        let pairs = food_list.match_ingredients_allergens();
        let string = FoodList::ingredient_string(&pairs);
        assert_eq!(string, "mxmxvkd,sqjhc,fvjkl".to_string());
    }
}
