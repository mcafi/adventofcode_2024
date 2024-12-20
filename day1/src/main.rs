use common::read_input;

fn main() {
    let lines = read_input();

    let (mut list_a, mut list_b) = parse_input(lines);

    list_a.sort();
    list_b.sort();

    println!("Result: {}", first_part(list_a.clone(), list_b.clone()));
    println!("Result: {}", second_parte(list_a.clone(), list_b.clone()));

}

fn parse_input(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Could not parse number"))
            .collect();
        list_a.push(numbers[0]);
        list_b.push(numbers[1]);
    }

    (list_a, list_b)
}

fn first_part(list_a: Vec<i32>, list_b: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut result = 0;
    while i < list_a.len() {
        result += (list_a[i] - list_b[i]).abs();
        i += 1;
    }

    result
}

fn second_parte(list_a: Vec<i32>, list_b: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for n in list_b {
        let count = map.entry(n).or_insert(0);
        *count += n;
    }

    for n in list_a {
        if map.contains_key(&n) {
            result += map.get(&n).unwrap();
        }
    }

    result
}
