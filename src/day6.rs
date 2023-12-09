pub fn solve(inputs: Vec<String>) {
    let time = inputs[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let distance = inputs[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    dbg!(time, distance);
    let mut way_c = 0;
    for i in 0..time {
        let speed = i;
        let rem_time = time - i;
        let dist = speed * rem_time;

        if dist > distance {
            way_c += 1;
        }
    }

    dbg!(way_c);
}
