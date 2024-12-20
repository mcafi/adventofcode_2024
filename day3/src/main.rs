use regex::Regex;

fn main() {
    let lines = common::read_input();
    let mut result = 0;
    for line in lines.clone() {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        for cap in re.captures_iter(&line) {
            result += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        }
    }

    println!("Result: {}", result);
    
    result = 0;
    
    let mut enabled = true;
    for line in lines.clone() {
        let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();

        for cap in re.captures_iter(&line) {
            if &cap[0] == "don't()" { 
                enabled = false;
            } else if &cap[0] == "do()" {
                enabled = true;
            } else if enabled {
                result += cap[2].parse::<i32>().unwrap() * cap[3].parse::<i32>().unwrap();
            }
        }
    }
    
    println!("Result: {}", result);
}
