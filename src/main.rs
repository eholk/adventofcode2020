mod day1;

fn main() {
    let input = day1::read_input(std::io::BufReader::new(std::io::stdin())).unwrap();
    let result = day1::solve_part2(input.as_slice());
    println!("{}", result);
}
