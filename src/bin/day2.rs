use advent_of_code2025::fetch_input;
use divisors::get_divisors;
use num_prime::nt_funcs::is_prime64;
use std::collections::HashSet;

#[tokio::main]
async fn main() {
    let input = fetch_input(2).await;
    let input_vec: Vec<&str> = input.split(',').collect();
    part_one(&input_vec);
    println!("-------------------");
    part_two(&input_vec);
}

fn parse_interval(interval: &str) -> (&str, &str) {
    let parts: Vec<&str> = interval.split('-').collect();
    let start: &str = parts[0].trim();
    let end: &str = parts[1].trim();
    (start, end)
}

fn all_parts_equal(s: &str, n: usize) -> bool {
    let len = s.len();
    if len % n != 0 {
        return false;
    }
    let part_len = len / n;
    let part = &s[..part_len];
    for i in 1..n {
        if &s[i * part_len..(i + 1) * part_len] != part {
            return false;
        }
    }
    true
}

fn sum_repeated_cycled_numbers_in_interval(start: &str, end: &str, n: usize) -> HashSet<i128> {
    let start_num: i128 = start.parse().unwrap();
    let end_num: i128 = end.parse().unwrap();
    let mut to_sum: HashSet<i128> = HashSet::new();

    let start_num_str = start_num.to_string();
    let len = start_num_str.len();
    let mid = len / n;
    let mut num: i128 = start_num;
    if len % n == 0 {
        num = start_num_str[..mid].repeat(n).parse().unwrap();
    }

    while num <= end_num {
        let num_str = num.to_string();
        let len = num_str.len();
        if len % n != 0 {
            let even_len = len + 1;
            let even_half_len = (even_len / 2) as u32;
            let first_half = 10_i128.pow(even_half_len - 1) as i128;
            num = first_half.to_string().repeat(n).parse().unwrap();
            continue;
        }
        let half_len = len / n;
        if num >= start_num && all_parts_equal(&num_str, n) {
            print!("{} ", num);
            to_sum.insert(num);
        }
        let first_half: i128 = num_str[..half_len].parse::<i128>().unwrap() + 1;
        let full_num_str: String = first_half.to_string().repeat(n).parse().unwrap();
        num = full_num_str.parse().unwrap();
    }
    if to_sum.is_empty() {
        println!("No invalid Ids found in interval.");
    } else {
        println!();
    }
    to_sum
}

fn part_one(intervals: &Vec<&str>) -> i128 {
    let mut total_sum: i128 = 0;
    for interval in intervals {
        let (start, end) = parse_interval(interval);
        println!("Start: {}, End: {} ", start, end);
        total_sum += sum_repeated_cycled_numbers_in_interval(start, end, 2)
            .iter()
            .sum::<i128>();
    }
    println!("Total sum of all intervals: {}", total_sum);
    total_sum
}

fn part_two(intervals: &Vec<&str>) -> i128 {
    let mut total_sum: i128 = 0;
    for interval in intervals {
        let (start, end) = parse_interval(interval);
        println!("Start: {}, End: {} ", start, end);
        let mut divisors: HashSet<u64> = get_divisors(start.len() as u64).into_iter().collect();
        divisors.extend(get_divisors(end.len() as u64));
        divisors.extend([start.len() as u64, end.len() as u64].into_iter());
        divisors = divisors.into_iter().filter(|p| is_prime64(*p)).collect();
        print!("Divisors to check: {:?} \n", divisors);
        let mut current_invalids: HashSet<i128> = HashSet::new();
        for n in divisors {
            current_invalids.extend(sum_repeated_cycled_numbers_in_interval(
                start, end, n as usize,
            ));
        }
        total_sum += current_invalids.iter().sum::<i128>();
        print!("Current total sum so far: {} \n", total_sum);
    }
    println!("Total sum of all intervals: {}", total_sum);
    total_sum
}
