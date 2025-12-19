use advent_of_code2025::fetch_input;

#[tokio::main]
async fn main() {
    let input = fetch_input(4).await;
    let mut paper_rolls: Vec<Vec<char>> = Vec::new();
    for paper_file in input.lines() {
        paper_rolls.push(parse_paper_files(paper_file));
    }
    part_one(paper_rolls.clone());
    println!("----------------");
    part_two(paper_rolls.clone());
}

fn parse_paper_files(paper_rolls: &str) -> Vec<char> {
    paper_rolls.chars().collect()
}

fn check_neighbors(paper_rolls: &mut Vec<Vec<char>>, i: usize, j: usize, max: usize) -> bool {
    let directions = [
        (0, 1),
        (1, 0),
        (1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut current_count = 0;
    for (di, dj) in directions.iter() {
        let new_i = i as isize + di;
        let new_j = j as isize + dj;
        if new_i >= 0
            && new_i < paper_rolls.len() as isize
            && new_j >= 0
            && new_j < paper_rolls[0].len() as isize
        {
            if paper_rolls[new_i as usize][new_j as usize] == '@'
                || paper_rolls[new_i as usize][new_j as usize] == '#'
            {
                current_count += 1;
                if current_count >= max {
                    return false;
                }
            }
        }
    }
    paper_rolls[i][j] = '#';
    true
}

fn clean_up(paper_rolls: &mut Vec<Vec<char>>) {
    for i in 0..paper_rolls.len() {
        for j in 0..paper_rolls[i].len() {
            if paper_rolls[i][j] == '#' {
                paper_rolls[i][j] = '.';
            }
        }
    }
}

fn part_one(mut paper_rolls: Vec<Vec<char>>) -> i32 {
    let mut amount_movable_papers = 0;
    for i in 0..paper_rolls.len() {
        for j in 0..paper_rolls[i].len() {
            if paper_rolls[i][j] == '@' || paper_rolls[i][j] == '#' {
                if check_neighbors(&mut paper_rolls, i, j, 4) {
                    amount_movable_papers += 1;
                }
            }
        }
    }
    println!("Amount of movable papers: {}", amount_movable_papers);
    amount_movable_papers
}

fn print_current_state(paper_rolls: &Vec<Vec<char>>) {
    for i in 0..paper_rolls.len() {
        for j in 0..paper_rolls[i].len() {
            print!("{}", paper_rolls[i][j]);
        }
        println!();
    }
    println!("----------------");
}

fn part_two(mut paper_rolls: Vec<Vec<char>>) -> i32 {
    let mut amount_movable_papers = 0;
    let mut current_movable_papers = -1;
    while current_movable_papers != 0 {
        current_movable_papers = 0;
        for i in 0..paper_rolls.len() {
            for j in 0..paper_rolls[i].len() {
                if paper_rolls[i][j] == '@' || paper_rolls[i][j] == '#' {
                    if check_neighbors(&mut paper_rolls, i, j, 4) {
                        current_movable_papers += 1;
                    }
                }
            }
        }
        println!(
            "Current movable papers this round: {}",
            current_movable_papers
        );
        //print_current_state(&paper_rolls);
        amount_movable_papers += current_movable_papers;
        clean_up(&mut paper_rolls);
    }
    println!(
        "Amount of movable papers part two: {}",
        amount_movable_papers
    );
    amount_movable_papers
}
