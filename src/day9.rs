fn vec_diff(v: Vec<i32>) -> Vec<i32> {
    let mut new: Vec<i32> = Vec::new();

    for i in 1..v.len() {
        new.push(v[i] - v[i - 1]);
    }
    new
}

fn calc_prefix(v: Vec<&i32>) -> i32 {
    let mut curr = 0;
    for val in &v {
        curr = **val - curr;
    }

    curr
}

pub fn solve(input: Vec<String>) {
    let lines: Vec<Vec<i32>> = input
        .iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut res: Vec<i32> = Vec::new();
    let mut part2: Vec<i32> = Vec::new();
    for line in &lines {
        let mut history: Vec<Vec<i32>> = Vec::new();
        history.push(line.clone());
        let mut tmp = line.clone();
        while history.last().unwrap().iter().sum::<i32>() != 0 {
            tmp = vec_diff(tmp.clone());

            history.push(tmp.clone());
        }

        res.push(history.iter().map(|x| x.last().unwrap()).sum::<i32>());
        let mut firsts: Vec<&i32> = history.iter().map(|x| x.first().unwrap()).collect();
        firsts.reverse();
        part2.push(calc_prefix(firsts));
    }

    println!("Part 1:{:?}", res.iter().sum::<i32>());
    println!("Part 2:{:?}", part2.iter().sum::<i32>());
}
