use advent_of_code2025::fetch_input;

#[tokio::main]
async fn main() {
    let input = fetch_input(1).await;
    part_one(&input).await;
    println!("---");
    part_two(&input).await;
}

async fn part_one(input: &str) {
    let mut current = 50;
    let mut counter_over_zeroes = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let mut value: i32 = line[1..].parse().unwrap();
        if direction == 'L' {
            value *= -1;
        }
        current += value;
        current = current.rem_euclid(100);
        if current == 0 {
            counter_over_zeroes += 1;
        }
    }
    println!("Final position: {}", current);
    println!("Number of times over 0: {}", counter_over_zeroes);
}

async fn part_two(input: &str) {
    let mut current: i32 = 50;
    let mut visited_zeroes = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let mut value: i32 = line[1..].parse().unwrap();
        if direction == 'L' {
            value *= -1;
        }

        let previous = current;
        current += value;

        visited_zeroes += (current.div_euclid(100) - previous.div_euclid(100)).abs();
        
        current = current.rem_euclid(100);
    }
    println!("Final position: {}", current);
    println!("Number of visited 0s: {}", visited_zeroes);
}
