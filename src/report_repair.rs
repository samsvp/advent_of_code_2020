use crate::utils;

/// Returns the multiplication of the years in the inputs/1.txt file
/// that sums to 2020 
pub fn report_repair() -> i32 {
    let target_year: i32 = 2020;
    
    let lines: Vec<String> = utils::get_lines("inputs/1.txt");
    let years: Vec<i32> = lines.iter().map(|l| l.parse::<i32>().unwrap()).collect();
    for i in 1..years.len() {
        let year1: i32 = years[i-1];
        let year2: i32 = *years[i..].iter().find(|&&yr| yr + year1 == target_year).unwrap_or(&0);
        if year2 != 0 {
            return year1 * year2;
        }
    }
    return 0;
}