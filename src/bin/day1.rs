use advent_of_code2025::fetch_input;

#[tokio::main]
async fn main() {
    let input = fetch_input(1).await;
    part_one(&input);
    println!("---");
    part_two(&input);
}

fn part_one(input: &str) -> i32 {
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
    counter_over_zeroes
}

fn part_two(input: &str) -> i32 {
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

        let mut crossings = (current.div_euclid(100) - previous.div_euclid(100)).abs();
        if direction == 'L' {
            if previous == 0 {
                crossings -= 1
            }
            if current.rem_euclid(100) == 0 {
                crossings += 1;
            }
        }
        visited_zeroes += crossings;

        current = current.rem_euclid(100);
    }
    println!("Final position: {}", current);
    println!("Number of visited 0s: {}", visited_zeroes);
    visited_zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l150_should_be_2() {
        let result = part_two("L150");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_r150_should_be_2() {
        let result = part_two("R150");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_r_start_at_0() {
        let result = part_two("L50\nR100");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_l_start_at_0() {
        let result = part_two("L50\nL100");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_100() {
        let result = part_two("L100\nR50");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_part_two() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let result = part_two(input);
        assert_eq!(result, 6);
    }
}
