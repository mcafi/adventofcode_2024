static SECURITY_THRESHOLD: i32 = 3;

fn main() {
    let lines = common::read_input();

    let numbers = parse_input(lines);

    println!("Result: {}", part1(numbers.clone()));
    println!("Result: {}", part2(numbers.clone()));
}

fn part1(numbers: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for row in numbers {
        let mut increasing = true;
        let mut safe = true;
        for i in 0..row.len() - 1 {
            if i == 0 && row[i] > row[i + 1] {
                increasing = false;
            }

            if !check_valid_sequence(row[i], row[i + 1], increasing) {
                safe = false;
                break;
            }
        }
        if safe {
            total += 1;
        }
    }
    total
}

fn part2(numbers: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for row in numbers {
        let mut increasing = true;
        let mut safe = true;
        let mut error = false;
        for i in 0..row.len() - 1 {
            if i == 0 && row[i] > row[i + 1] {
                increasing = false;
            }

            if !check_valid_sequence(row[i], row[i + 1], increasing) {
                if error {
                    safe = false;
                    break;
                } else {
                    error = true;
                }
            }
        }
        if safe {
            total += 1;
        }
    }
    total
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut list: Vec<Vec<i32>> = vec![];

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Could not parse number"))
            .collect();
        list.push(numbers);
    }

    list
}

fn check_valid_sequence(number_a: i32, number_b: i32, increasing: bool) -> bool {
    if increasing && number_b <= number_a || !increasing && number_b >= number_a {
        return false;
    }
    (number_a - number_b).abs() <= SECURITY_THRESHOLD
}
