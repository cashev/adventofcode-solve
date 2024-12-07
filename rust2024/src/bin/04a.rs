fn main() {
    let line = INPUT;
    println!("{}", search_xmas(line));
}

fn search_xmas(lines: &str) -> i32 {
    let mut count = 0;
    let lines: Vec<&str> = lines.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    // 横方向のチェック
    for y in 0..height {
        for x in 0..width - 3 {
            let s: String = chars[y][x..x + 4].iter().collect();
            if s == "XMAS" || s == "SAMX" {
                count += 1;
            }
        }
    }

    // 縦方向のチェック
    for x in 0..width {
        for y in 0..height - 3 {
            let s: String = (0..4).map(|i| chars[y + i][x]).collect();
            if s == "XMAS" || s == "SAMX" {
                count += 1;
            }
        }
    }

    // 斜め方向のチェック(右下)
    for y in 0..height - 3 {
        for x in 0..width - 3 {
            let s: String = (0..4).map(|i| chars[y + i][x + i]).collect();
            if s == "XMAS" || s == "SAMX" {
                count += 1;
            }
        }
    }

    // 斜め方向のチェック(左下)
    for y in 0..height - 3 {
        for x in 3..width {
            let s: String = (0..4).map(|i| chars[y + i][x - i]).collect();
            if s == "XMAS" || s == "SAMX" {
                count += 1;
            }
        }
    }
    count
}

const INPUT: &str = "";
