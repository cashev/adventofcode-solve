fn main() {
    let line = INPUT;

    let mut sum = 0;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut enabled = true;
    while i < chars.len() {
        if enabled {
            if i + 3 < chars.len()
                && chars[i] == 'm'
                && chars[i + 1] == 'u'
                && chars[i + 2] == 'l'
                && chars[i + 3] == '('
            {
                let mut j = i + 4;
                let mut first_num = String::new();
                while j < chars.len() && chars[j].is_numeric() {
                    first_num.push(chars[j]);
                    j += 1;
                }
                if j < chars.len() && chars[j] == ',' {
                    j += 1;
                    let mut second_num = String::new();
                    while j < chars.len() && chars[j].is_numeric() {
                        second_num.push(chars[j]);
                        j += 1;
                    }
                    if j < chars.len() && chars[j] == ')' {
                        if let (Ok(n1), Ok(n2)) =
                            (first_num.parse::<i32>(), second_num.parse::<i32>())
                        {
                            sum += n1 * n2;
                        }
                    }
                }
            }
        }
        if i + 3 < chars.len()
            && chars[i] == 'd'
            && chars[i + 1] == 'o'
            && chars[i + 2] == '('
            && chars[i + 3] == ')'
        {
            enabled = true;
        }
        if i + 6 < chars.len()
            && chars[i] == 'd'
            && chars[i + 1] == 'o'
            && chars[i + 2] == 'n'
            && chars[i + 3] == '\''
            && chars[i + 4] == 't'
            && chars[i + 5] == '('
            && chars[i + 6] == ')'
        {
            enabled = false;
        }
        i += 1;
    }
    println!("{}", sum);
}

const INPUT: &str = "";
