pub fn solve1(input: Vec<String>) {
    let mut nums: Vec<Vec<u32>> = vec![];
    for strings in input {
        nums.push(strings.chars().filter_map(|x| x.to_digit(10)).collect());
    }

    let mut sum: u32 = 0;
    for val in nums {
        let f = val[0];
        let e = val.last().unwrap();

        sum += f * 10 + e;
    }

    println!("{}", sum);
}

fn num_replace(val: &str) -> String {
    val.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}
pub fn solve2(input: Vec<String>) {
    let inputs: Vec<String> = input.iter().map(|x| num_replace(x)).collect();
    solve1(inputs);
}
