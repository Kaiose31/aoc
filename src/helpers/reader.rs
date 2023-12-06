use std::fs::read_to_string;

pub fn read_input(filename: String) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn read_new(filename: &str) -> String {
    read_to_string(filename).unwrap()
}
