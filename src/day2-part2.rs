mod line_reader;

fn main() {
    let lines = line_reader::vec_from_files("inputs/day2-input.txt");
    let mut aim = 0;
    let mut depth = 0;
    let mut horiz_pos = 0;

    for line in lines {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let magnitude = split.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "up" => aim = aim - magnitude,
            "down" => aim = aim + magnitude,
            "forward" => {
                horiz_pos += magnitude;
                depth += magnitude * aim;
            },
            _ => {}
        }
    }

    println!("Total magnitude: {:?}", depth * horiz_pos);
}
