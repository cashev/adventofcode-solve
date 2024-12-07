fn main() {
    let input_lines = INPUT.lines();
    let mut lines: Vec<Vec<i32>> = Vec::new();
    for line in input_lines {
        let line_numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        lines.push(line_numbers);
    }

    let mut sum = 0;
    for line in lines {
        if is_safe(&line) {
            // println!("{:?}", line);
            sum += 1;
        } else {
            for i in 0..line.len() {
                let mut new_line = line.clone();
                new_line.remove(i);
                if is_safe(&new_line) {
                    sum += 1;
                    break;
                }
            }
        }
    }

    println!("{}", sum);
}

fn is_safe(line: &Vec<i32>) -> bool {
    let is_inc = line[0] <= line[1];
    if is_inc {
        for i in 1..line.len() {
            let prev = line[i - 1];
            let current = line[i];
            if prev >= current {
                return false;
            }
            if current - prev > 3 {
                return false;
            }
        }
    } else {
        for i in 1..line.len() {
            let prev = line[i - 1];
            let current = line[i];
            if prev <= current {
                return false;
            }
            if prev - current > 3 {
                return false;
            }
        }
    }
    true
}

const INPUT: &str = "";
