use advent_of_code2025::fetch_input;

#[tokio::main]
async fn main() {
    let input = fetch_input(3).await;
    let mut banks: Vec<Vec<u32>> = Vec::new();
    for bank in input.lines() {
        banks.push(parse_bank(bank));
    }
    part_one(&banks);
    println!("----------------");
    part_two(&banks);
}

fn parse_bank(bank: &str) -> Vec<u32> {
    bank.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn convert_slice_to_num(slice: &[u32]) -> u64 {
    let mut num: u64 = 0;
    for &digit in slice {
        num = num * 10 + digit as u64;
    }
    num
}

fn get_max_jolts(bank: Vec<u32>, n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut i = 0;
    let mut j = n as usize;
    let mut max_i = 0;
    let mut current_max = 0;
    while j <= bank.len() {
        let slice = &bank[i..j as usize];
        let new_max = current_max.max(convert_slice_to_num(slice));
        if new_max > current_max  && (new_max / 10u64.pow(n - 1)) > (current_max / 10u64.pow(n - 1)){
            current_max = new_max;
            max_i = i;
        }
        i += 1;
        j += 1;
    }
    let most_left_val = (current_max / 10u64.pow(n - 1)) * 10u64.pow(n - 1);
    //println!("current_max: {}, most_left_val: {}, max_i: {}", current_max, most_left_val, max_i);
    current_max =
        current_max.max(most_left_val + get_max_jolts(bank[(max_i+1)..].to_vec(), n - 1));
    println!("current_max final: {}, n: {}", current_max, n);
    current_max
}

fn part_one(banks: &Vec<Vec<u32>>) {
    let mut total_jolts: u64 = 0;
    for bank in banks {
        println!("Processing bank: {:?}", bank);
        total_jolts += get_max_jolts(bank.clone(), 2);
    }
    println!("Total jolts part one: {}", total_jolts);
}

fn part_two(banks: &Vec<Vec<u32>>) {
    let mut total_jolts: u64 = 0;
    for bank in banks {
        println!("Processing bank: {:?}", bank);
        total_jolts += get_max_jolts(bank.clone(), 12);
    }
    println!("Total jolts part two: {}", total_jolts);
}
