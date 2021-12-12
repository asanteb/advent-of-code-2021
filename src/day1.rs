mod line_reader;

fn main() {
    let lines = line_reader::vec_from_files("src/day1-input.txt");
    let mut last_number = lines[0].parse::<i32>().unwrap();
    let mut increase_amount = 0;

    for (idx, line) in lines.iter().enumerate() {
        let num = line.parse::<i32>().unwrap();

        if idx == 0 {
            last_number = num;
        } else {
            match num {
                x if x  > last_number => {
                    increase_amount += 1;
                    last_number = num;
                }
                _ => {
                    last_number = num;
                }
            }
        }

    }
    println!("Number of increases {:?}", increase_amount);
}
