use std::{env, fs};

pub fn example(s: &str) -> String {
    fs::read_to_string(&format!("examples/{}", s)).unwrap()
}
