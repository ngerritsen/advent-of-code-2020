use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

type AllergenMap = HashMap<String, Vec<String>>;
type Counts = HashMap<String, usize>;

fn main() {
    let foods = parse_foods(include_str!("input.txt"));
    let ingredients = get_ingredient_counts(&foods);
    let potential_allergen_map = get_potential_allergen_map(&foods, &ingredients);
    let possible_allergen_map = get_possible_allergen_map(&potential_allergen_map, &foods);

    println!(
        "{}",
        count_ingredients_without_allergens(&possible_allergen_map, &ingredients)
    );

    println!("{}", get_settled(&possible_allergen_map).join(","));
}

fn count_ingredients_without_allergens(
    possible_allergen_map: &AllergenMap,
    ingredients: &Counts,
) -> usize {
    possible_allergen_map
        .iter()
        .filter(|(_, allergens)| allergens.is_empty())
        .map(|(ingredient, _)| ingredients.get(ingredient).unwrap())
        .sum()
}

fn get_settled(possible_allergen_map: &AllergenMap) -> Vec<String> {
    let mut settled_ingredients: HashMap<String, String> = HashMap::new();
    let mut possible_allergen_map: AllergenMap = possible_allergen_map
        .iter()
        .filter(|(_, allergens)| !allergens.is_empty())
        .map(|(ingredient, allergens)| (ingredient.clone(), allergens.clone()))
        .collect();

    while settled_ingredients.len() < possible_allergen_map.len() {
        let (ingredient, allergens) = possible_allergen_map
            .iter()
            .filter(|(ingredient, _)| !settled_ingredients.contains_key(*ingredient))
            .find(|(_, allergens)| allergens.len() == 1)
            .unwrap();

        let allergen = allergens.iter().next().unwrap().clone();
        settled_ingredients.insert(ingredient.clone(), allergen.clone());

        possible_allergen_map = possible_allergen_map
            .iter()
            .map(|(ingredient, allergens)| {
                (
                    ingredient.clone(),
                    allergens
                        .iter()
                        .filter(|a| allergens.len() == 1 || allergen != **a)
                        .cloned()
                        .collect(),
                )
            })
            .collect::<HashMap<_, _>>();
    }

    settled_ingredients
        .iter()
        .sorted_by(|(_, allergen_a), (_, allergen_b)| Ord::cmp(&allergen_a, &allergen_b))
        .map(|(ingredient, _)| ingredient.clone())
        .collect()
}

fn get_possible_allergen_map(potential_allergen_map: &AllergenMap, foods: &[Food]) -> AllergenMap {
    potential_allergen_map
        .iter()
        .map(|(ingredient, allergens)| {
            (
                ingredient.clone(),
                allergens
                    .iter()
                    .filter(|allergen| {
                        foods
                            .iter()
                            .filter(|food| food.allergens.contains(allergen))
                            .all(|food| food.ingredients.contains(ingredient))
                    })
                    .cloned()
                    .collect(),
            )
        })
        .collect()
}

fn get_potential_allergen_map(foods: &[Food], ingredients: &Counts) -> AllergenMap {
    ingredients
        .iter()
        .map(|(ingredient, _)| {
            (
                ingredient.clone(),
                foods
                    .iter()
                    .filter(|food| food.ingredients.contains(ingredient))
                    .flat_map(|food| food.allergens.clone())
                    .unique()
                    .collect(),
            )
        })
        .collect()
}

fn get_ingredient_counts(foods: &[Food]) -> Counts {
    foods
        .iter()
        .flat_map(|food| food.ingredients.clone())
        .counts()
}

fn parse_foods(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|l| {
            let (ingredients_input, allergens_input) =
                l.split(" (contains ").collect_tuple().unwrap();

            Food {
                ingredients: ingredients_input
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect(),
                allergens: allergens_input
                    .trim_end_matches(')')
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect(),
            }
        })
        .collect()
}
