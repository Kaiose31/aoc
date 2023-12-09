use std::collections::HashMap;

pub fn solve(inputs: Vec<String>) {
    let mut cp = inputs.clone();
    let choices = cp.remove(0);
    cp.remove(0);
    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();

    for x in &cp {
        let (key, val) = x.split_once(" = ").unwrap();

        network.insert(key, val[1..9].split_once(", ").unwrap());
    }

    // State machine
    let mut current_node = "AAA";
    let mut counter = 0;

    'outer: loop {
        for choice in choices.chars() {
            if current_node == "ZZZ" {
                break 'outer;
            }
            match choice {
                'R' => current_node = network.get(&current_node).unwrap().1,
                'L' => current_node = network.get(&current_node).unwrap().0,
                _ => {}
            }
            counter += 1;
        }
    }

    dbg!(counter);
}
