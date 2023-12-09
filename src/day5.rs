use itertools::Itertools;
pub fn solve(inputs: String) {
    let mut sections: Vec<&str> = inputs.split("\n\n").collect();
    let mut maps: Vec<(&str, Vec<Vec<i64>>)> = Vec::new();
    let seeds: Vec<i64> = sections
        .remove(0)
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut new_seeds: Vec<i64> = Vec::new();
    for (prev, next) in seeds.clone().into_iter().tuples() {
        for i in prev..prev + next {
            new_seeds.push(i);
        }
    }

    println!("Seeds done {}", new_seeds.len());

    //process maps
    for line in sections {
        let (name, rest) = line.split_once(":\n").unwrap();
        let values: Vec<Vec<i64>> = rest
            .split("\n")
            .map(|x| {
                x.split(" ")
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect()
            })
            .collect();

        maps.push((name, values));
    }
    // After parsing go through each seed and each map for each seed
    let mut res: Vec<i64> = Vec::new();
    for seed in new_seeds.iter_mut() {
        //dst, source, len.
        let curr_val = seed;
        for (_, mapping) in maps.iter() {
            for row in mapping {
                let dst_start = row[0];
                let s_start = row[1];
                let range = row[2];

                if *curr_val >= s_start && *curr_val < s_start + range {
                    let iv = *curr_val - s_start;

                    let new_val = dst_start + iv;

                    *curr_val = new_val;
                    break;
                }
            }
            // println!("updated value in each map {} : {}", name, &curr_val);
        }
        res.push(*curr_val);
    }
    dbg!(res.iter().min());
}
