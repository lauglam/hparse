pub fn example(s: &str) -> String {
    std::fs::read_to_string(&format!("examples/{}", s)).unwrap()
}
