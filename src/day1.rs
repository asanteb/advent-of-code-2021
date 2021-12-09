mod line_reader;

fn main() {
    let lines = line_reader::vec_from_files("src/day1-input.txt");
    for line in lines {
        println!("{:?}", line);
    }
}
