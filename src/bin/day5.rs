use advent_of_code2025::fetch_input;

#[tokio::main]
async fn main() {
    let input = fetch_input(5).await;
    /*     let input = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32"; */
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = get_ranges(ranges_str);
    let ingredients = get_ingredients(ingredients_str);

    part_one(ranges.clone(), ingredients);
    println!("----------------");
    part_two(ranges.clone());
}

fn get_ranges(range_str: &str) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for range in range_str.lines() {
        let (start, end) = range.split_once('-').unwrap();
        ranges.push((start.parse().unwrap(), end.parse().unwrap()));
    }
    ranges
}

fn get_ingredients(ingredients_str: &str) -> Vec<u64> {
    let mut ingredients: Vec<u64> = Vec::new();
    for ingredient in ingredients_str.lines() {
        ingredients.push(ingredient.parse().unwrap());
    }
    ingredients
}

fn part_one(ranges: Vec<(u64, u64)>, ingredients: Vec<u64>) -> u64 {
    let mut count_fresh_ingredients = 0;
    for ingredient in ingredients {
        let mut is_fresh = false;
        for (start, end) in &ranges {
            if ingredient >= *start && ingredient <= *end {
                is_fresh = true;
                break;
            }
        }
        if is_fresh {
            count_fresh_ingredients += 1;
        }
    }
    println!("Fresh ingredients count: {}", count_fresh_ingredients);
    count_fresh_ingredients
}

fn process_range(ranges: &mut Vec<(u64, u64)>) {
    ranges.sort_by_key(|&(start, _)| start);
    println!("Sorted: {:?}", ranges);
    let mut i = 0;

    while i < ranges.len() - 1 {
        if ranges[i].1 >= ranges[i + 1].0 {
            if ranges[i].1 < ranges[i + 1].1 {
                ranges[i].1 = ranges[i + 1].1;
            }
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
    println!("Processed: {:?}", ranges);
}

fn count_amout_elems_in_ranges(ranges: Vec<(u64, u64)>) -> u64 {
    let mut result = 0;
    for range in ranges {
        result += range.1 - range.0 + 1;
    }
    result
}

fn part_two(mut ranges: Vec<(u64, u64)>) -> u64 {
    println!("{:?}", ranges);
    process_range(&mut ranges);
    let result = count_amout_elems_in_ranges(ranges);
    println!("The final count is: {}", result);
    result
}
