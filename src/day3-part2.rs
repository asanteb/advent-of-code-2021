mod line_reader;

fn main() {
    let lines = line_reader::vec_from_files("inputs/day3-input.txt");
    let mut oxygen_vec: Vec<String> = lines.clone();
    let mut co2_vec: Vec<String> = lines.clone();
    let binary_length = lines[0].len();

    for i in 0..binary_length {
        let oxygen_total: i32 = oxygen_vec.iter().map(|line| {
            return line.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
        }).sum();
        let co2_total: i32 = co2_vec.iter().map(|line| {
            return line.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
        }).sum();

        let oxygen_bit = if oxygen_total >= (oxygen_vec.len() as i32) - oxygen_total {1} else {0};
        let co2_bit = if co2_total >= (co2_vec.len() as i32) - co2_total {0} else {1};

        if oxygen_vec.len() > 1 {
            oxygen_vec = oxygen_vec.into_iter().filter(|line| line.chars().nth(i) == char::from_digit(oxygen_bit as u32, 10)).collect();
        }

        if co2_vec.len() > 1 {
            co2_vec = co2_vec.into_iter().filter(|line| line.chars().nth(i) == char::from_digit(co2_bit as u32, 10)).collect();
        }
    }

    let oxygen_rating = isize::from_str_radix(&oxygen_vec[0], 2).unwrap();
    let co2_rating = isize::from_str_radix(&co2_vec[0], 2).unwrap();
    println!("Life support rating: {:?}", oxygen_rating * co2_rating);
}
