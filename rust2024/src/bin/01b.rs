use std::collections::HashMap;

fn main() {
    let input_lines = INPUT.lines();
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for line in input_lines {
        let mut numbers = line.split_whitespace();
        if let Some(left) = numbers.next() {
            if let Ok(num) = left.parse::<i32>() {
                left_numbers.push(num);
            }
        }
        if let Some(right) = numbers.next() {
            if let Ok(num) = right.parse::<i32>() {
                right_numbers.push(num);
            }
        }
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    for right in right_numbers {
        let count = map.entry(right).or_insert(0);
        *count += 1;
    }

    let mut sum = 0;
    for left in left_numbers {
        let count = map.entry(left).or_insert(0);
        sum += left * *count;
    }

    println!("{}", sum);
}

const INPUT: &str = "";
