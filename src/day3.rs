mod line_reader;

fn main() {
    let lines = line_reader::vec_from_files("inputs/day3-input.txt");
    let binary_length = lines[0].len();
    let lines_length = lines.len() as i32;
    let mut gamma_rate: String = "".to_string();
    let mut epsilon_rate: String = "".to_string();

    for i in 0..binary_length {
        let total: i32 = lines.iter().map(|line| {
            return line.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
        }).sum();

        if total > lines_length / 2 {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        } else {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }

    }
    let decimal_gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let decimal_epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("Power Consumption: {:?}", decimal_gamma_rate * decimal_epsilon_rate);
}
