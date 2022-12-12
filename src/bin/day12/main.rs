fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day12/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    
}