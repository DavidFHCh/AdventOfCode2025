use advent_of_code2025::fetch_input;

#[tokio::main]
async fn main() {
    println!("Fetching Day 1 input...");
    let input = fetch_input(1).await;

    let mut current = 50;
    let mut counter_over_zeroes = 0;
    
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let mut value: i32 = line[1..].parse().unwrap();
        if direction == 'L' {
            value *= -1;
            value += 100;
        }
        //println!("Moving {} by {}", direction, value);
        current += value;
        current = current % 100;
        if current == 0 {
            counter_over_zeroes += 1;
        }
    }
    println!("Final position: {}", current);
    println!("Number of times over 0: {}", counter_over_zeroes);
}
