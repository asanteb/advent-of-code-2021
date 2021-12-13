mod line_reader;

fn main() {
    let lines = line_reader::vec_from_files("inputs/day2-input.txt");
    let mut depth = 0;
    let mut horiz_pos = 0;

    for line in lines {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let magnitude = split.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "up" => depth = depth - magnitude,
            "down" => depth = depth + magnitude,
            "forward" => horiz_pos += magnitude,
            _ => {}
        }
    }

    println!("Total magnitude: {:?}", depth * horiz_pos);
}
