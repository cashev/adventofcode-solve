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

    left_numbers.sort();
    right_numbers.sort();

    let mut sum = 0;
    for i in 0..left_numbers.len() {
        sum += (left_numbers[i] - right_numbers[i]).abs();
    }

    println!("{}", sum);
}

const INPUT: &str = "";
