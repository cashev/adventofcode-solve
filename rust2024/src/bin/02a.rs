fn main() {
    let input_lines = INPUT.lines();
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for line in input_lines {
        let line_numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        numbers.push(line_numbers);
    }

    let mut sum = 0;
    for line in numbers {
        let mut is_safe = true;
        let is_all_increasing = line.windows(2).all(|w| w[0] < w[1]);
        let is_all_decreasing = line.windows(2).all(|w| w[0] > w[1]);
        if is_all_increasing || is_all_decreasing {
            for i in 1..line.len() {
                let prev = line[i - 1];
                let current = line[i];
                if (prev - current).abs() > 3 {
                    is_safe = false;
                }
            }
        } else {
            is_safe = false;
        }
        if is_safe {
            sum += 1;
        }
    }

    println!("{}", sum);
}

const INPUT: &str = "";
