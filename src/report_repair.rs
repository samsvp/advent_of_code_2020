use crate::utils;

static TARGET_YEAR: i32 = 2020;

fn parse_lines_to_int() -> Vec<i32> {
    let lines: Vec<String> = utils::get_lines("inputs/1.txt");
    return lines.iter().map(|l| l.parse::<i32>().unwrap()).collect();
}

/// Returns the numbers in [values] that sums to [sum]
fn get_nums_where_sum(values: Vec<i32>, sum: i32) -> Vec<i32> {
    for i in 1..values.len() {
        let value1: i32 = values[i-1];
        let target = sum - value1;
        let value2: i32 = *values[i..].iter().find(|&&v| v == target).unwrap_or(&0);
        if value2 != 0 {
            return vec![value1, value2];
        }
    }
    return vec![0, 0];
}

/// Returns the multplication of the two numbers that sums to 2020
pub fn report_repair_pt1() -> i32 {
    let years: Vec<i32> = parse_lines_to_int();
    let nums: Vec<i32> = get_nums_where_sum(years, TARGET_YEAR);
    return nums.iter().fold(1, |mult, x| mult * x);
}

/// Returns the multplication of the three numbers that sums to 2020
pub fn report_repair_pt2() -> i32 {
    let years: Vec<i32> = parse_lines_to_int();
    for i in 0..years.len()-2 {
        let current_year = years[i];
        let nums: Vec<i32> = get_nums_where_sum(years[i+1..].to_vec(), TARGET_YEAR - current_year);
        if nums[0] != 0 && nums[1] != 0 {
            return nums.iter().fold(current_year, |mult, x| mult * x);
        } 
    }
    return 0;
}
