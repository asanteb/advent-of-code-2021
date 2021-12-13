mod line_reader;

fn main() {
    let lines = line_reader::vec_from_files("src/day1-input.txt");
    let length = lines.len();
    let mut last_number: i32 = 0;
    let mut increase_amount = 0;

    for i in 0..length {
        if i + 3 <= length {
            let window = &lines[i..i+3];
            let sum: i32 = window.iter().map(|x| x.parse::<i32>().unwrap()).sum();

            // Skip first window because there is nothing to compare it against
            if i > 0 && sum > last_number {
                increase_amount += 1;
            }
            last_number = sum;
        }
    }
    println!("Number of increases {:?}", increase_amount);
}
