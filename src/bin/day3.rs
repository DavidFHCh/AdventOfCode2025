use advent_of_code2025::fetch_input;
use std::collections::BinaryHeap;

#[tokio::main]
async fn main() {
    let input = fetch_input(3).await;
/*     let input = "987654321111111
811111111111119
234234234234278
818181911112111"; */
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

fn part_one(banks: &Vec<Vec<u32>>) -> u32 {
    let mut total_jolts = 0;
    for bank in banks {
        let mut max_heap: BinaryHeap<u32> = BinaryHeap::from(bank.clone());
        let max_value = max_heap.pop().unwrap();
        let mut curreent_jolts = 0;
        let indices = bank
            .iter()
            .enumerate()
            .filter(|&(_, &value)| value == max_value)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        if indices.len() < 2 {
            if indices[0] == bank.len() - 1 {
                let second_max = max_heap.pop().unwrap();
                curreent_jolts += (second_max * 10) + max_value;
            } else {
                let i = indices[0] + 1;
                let mut max_heap_rem: BinaryHeap<u32> =
                    bank[i..].into_iter().cloned().collect();
                let second_max = max_heap_rem.pop().unwrap();
                curreent_jolts += (max_value * 10) + second_max;
            }
        } else {
            curreent_jolts += (max_value * 10) + max_value
        }
        print!("{} ", curreent_jolts);
        total_jolts += curreent_jolts;
    }
    println!("Total jolts: {}", total_jolts);
    total_jolts
}

fn part_two(banks: &Vec<Vec<u32>>) {
    println!("Part two not implemented yet.");
}
