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

    for i in 0..height - 2 {
        for j in 0..width - 2 {
            let top_left = chars[i][j];
            let top_right = chars[i][j + 2];
            let bottom_left = chars[i + 2][j];
            let bottom_right = chars[i + 2][j + 2];
            let center = chars[i + 1][j + 1];
            if top_left == 'X'
                || top_right == 'X'
                || bottom_left == 'X'
                || bottom_right == 'X'
                || center == 'X'
            {
                continue;
            }
            if center != 'A' {
                continue;
            }

            if !(top_left == 'M' || top_left == 'S')
                || !(top_right == 'M' || top_right == 'S')
                || !(bottom_left == 'M' || bottom_left == 'S')
                || !(bottom_right == 'M' || bottom_right == 'S')
            {
                continue;
            }
            // let mut is_mas = true;
            if top_left == bottom_right || top_right == bottom_left {
                continue;
            }
            count += 1;
        }
    }
    count
}

const INPUT: &str = "";
