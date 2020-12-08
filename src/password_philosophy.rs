use crate::utils;

fn get_lines() -> Vec<String> {
    let lines: Vec<String> = utils::get_lines("inputs/2.txt");
    return lines;
}

fn get_splits_and_number(line: &String) -> (Vec<&str>, i16, i16) {
    let first_split: Vec<&str> = line.split('-').collect();
    let second_split: Vec<&str> = first_split[1].split(' ').collect();
    let lower: i16 = first_split[0].parse::<i16>().unwrap();
    let upper: i16 = second_split[0].parse::<i16>().unwrap();
    return (second_split, lower, upper);
}

fn validate_psw_1(line: &String) -> i32 {
    let (second_split, lower, upper) = get_splits_and_number(line);
    let count: i16 = second_split[2].matches(
        second_split[1].chars().next().unwrap()).count() as i16;
    return if count >= lower && count <= upper { 1 } else { 0 }; 
}

fn validate_psw_2(line: &String) -> i32 {
    let (second_split, lower, upper) = get_splits_and_number(line);
    let mut count: i32 = 0;

    let m_chr: char = *second_split[1].chars().nth(0).as_ref().unwrap();

    for (i, chr) in second_split[2].chars().enumerate() {
        if (i + 1 == lower as usize || i + 1 == upper as usize) && chr == m_chr {
            count += 1;
        }
    }

    return if count == 1 { 1 } else { 0 }; 
}

pub fn count_valid_psw_1() -> i32 {
    let lines: Vec<String> = get_lines();
    let x: i32 = lines.iter().map(|l| validate_psw_1(l)).sum();
    return x;
}

pub fn count_valid_psw_2() -> i32 {
    let lines: Vec<String> = get_lines();
    let x: i32 = lines.iter().map(|l| validate_psw_2(l)).sum();
    return x;
}
