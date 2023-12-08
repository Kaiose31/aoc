use crate::helpers::{read_input, read_new};
use std::collections::HashMap;
use std::ops::Index;
use std::{cmp::Ordering, iter::zip};
mod helpers {
    mod reader;
    pub use reader::read_input;
    pub use reader::read_new;
}
use itertools::{all, Itertools};
use regex::Regex;

fn day1(input: Vec<String>) {
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

fn day1two() {
    let input = helpers::read_input("inputs/day1.txt".to_string());
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
    let inputs: Vec<String> = input.iter().map(|x| num_replace(x)).collect();
    day1(inputs);
}

fn day2() {
    #[derive(Debug, Clone, Copy)]
    struct Game {
        id: u32,
        red: u32,
        blue: u32,
        green: u32,
    }
    impl Game {
        fn new(id: u32, red: u32, blue: u32, green: u32) -> Self {
            Game {
                id,
                red,
                blue,
                green,
            }
        }

        fn is_possible(self) -> bool {
            //12 red cubes, 13 green cubes, and 14 blue cubes.
            if self.red > 12 || self.green > 13 || self.blue > 14 {
                return false;
            }
            true
        }

        fn power(self) -> u32 {
            self.blue * self.green * self.red
        }
    }

    fn get_max_cube(cubes: Vec<(&str, &str)>) -> HashMap<String, u32> {
        let mut h: HashMap<String, u32> = HashMap::new();
        let mut max_b: Vec<u32> = vec![];
        let mut max_r: Vec<u32> = vec![];
        let mut max_g: Vec<u32> = vec![];

        for (value, color) in cubes {
            if color == "red" {
                max_r.push(value.parse().unwrap());
            } else if color == "green" {
                max_g.push(value.parse().unwrap());
            } else {
                max_b.push(value.parse().unwrap());
            }
        }
        h.insert("red".to_string(), *max_r.iter().max().unwrap());
        h.insert("green".to_string(), *max_g.iter().max().unwrap());
        h.insert("blue".to_string(), *max_b.iter().max().unwrap());

        h
    }

    let input = read_input("inputs/day2.txt".to_string());
    let mut games: Vec<Game> = vec![];

    for game in input {
        let (game_id, game_info) = game.split_once(":").unwrap();
        let cubes: Vec<_> = game_info
            .split(";")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| {
                x.split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|y| y.strip_prefix(" ").unwrap())
                    .collect::<Vec<&str>>()
            })
            .flatten()
            .collect::<Vec<&str>>()
            .iter()
            .map(|z| z.split_once(" "))
            .flatten()
            .collect();

        // dbg!(game_id, &cubes, &game);
        let max_cubes = get_max_cube(cubes);

        games.push(Game {
            id: game_id[5..].parse::<u32>().unwrap(),
            red: *max_cubes.get("red").unwrap(),
            blue: *max_cubes.get("blue").unwrap(),
            green: *max_cubes.get("green").unwrap(),
        });
    }
    let mut counter = 0;
    let mut p_counter = 0;
    for game in games {
        if game.is_possible() {
            counter += game.id;
        }
        p_counter += game.power();
    }

    println!("{} {}", counter, p_counter);
}

fn day3() {
    #[derive(Debug, Clone, Copy)]
    struct Number {
        num: i32,
        row: i32,
        start: i32,
        end: i32,
        counted: Option<bool>,
    }

    impl Number {
        fn is_collide(mut self, row: i32, pos: i32, _symbol: char) -> Self {
            let d1: i32;
            let d2: i32;

            // d1 = distance from start to symbol

            d1 = (row - self.row).abs() + (pos - self.start).abs();
            d2 = (row - self.row).abs() + (pos - self.end).abs();

            if row != self.row && (d1 <= 2 || d2 <= 2) {
                if (row - self.row).abs() >= 2 {
                    self.counted = Some(false);
                } else {
                    self.counted = Some(true);
                }
            } else if row == self.row && (d1 <= 1 || d2 <= 1) {
                self.counted = Some(true);
            } else {
                match self.counted {
                    Some(_v) => {}
                    None => self.counted = Some(false),
                }
            }

            Number {
                num: self.num,
                row: self.row,
                start: self.start,
                end: self.end,
                counted: self.counted,
            }
        }
    }

    impl PartialEq for Number {
        fn eq(&self, other: &Self) -> bool {
            if self.row == other.row && self.start == other.start {
                true
            } else {
                false
            }
        }
    }

    // true if adjacent symbol else false.
    let inputs = read_input("inputs/day3.txt".to_string());
    let re = Regex::new(r"\d+(\d+)?").unwrap();
    let mut nums: Vec<Number> = Vec::new();
    for (idx, line) in inputs.iter().enumerate() {
        for m in re.captures_iter(&line) {
            let location = m.get(0).unwrap();

            nums.push(Number {
                num: location.as_str().parse().unwrap(),
                row: idx as i32,
                start: location.start() as i32,
                end: location.end() as i32 - 1,
                counted: None,
            })
        }
    }

    let mut new_nums: Vec<Number> = Vec::new();
    for (row, line) in inputs.iter().enumerate() {
        for (pos, chr) in line.chars().enumerate() {
            if chr == '.' {
                continue;
            } else if chr.is_ascii_punctuation() {
                for n in nums.iter_mut() {
                    let x = n.is_collide(row as i32, pos as i32, chr);
                    match x.counted {
                        Some(v) => {
                            if v {
                                new_nums.push(x)
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }

    new_nums.dedup();
    dbg!(new_nums.len(), nums.len());

    dbg!(new_nums.iter().fold(0, |acc, x| acc + x.num));

    // part 2
    let mut new_gears: Vec<Vec<Number>> = Vec::new();

    for (row, line) in inputs.iter().enumerate() {
        for (pos, chr) in line.chars().enumerate() {
            if chr == '.' {
                continue;
            } else if chr == '*' {
                let mut gears = Vec::new();
                for n in nums.iter_mut() {
                    let x = n.is_collide(row as i32, pos as i32, chr);
                    match x.counted {
                        Some(v) => {
                            if v {
                                gears.push(x);
                            }
                        }
                        None => {}
                    }
                }
                new_gears.push(gears);
            }
        }
    }

    let mut total = 0;

    for gear in new_gears {
        if gear.len() == 2 {
            total += gear[0].num * gear[1].num;
        }
    }

    dbg!(total);
}

fn day4() {
    #[derive(Debug)]
    struct Card {
        winners: Vec<i32>,
        current: Vec<i32>,
        points: i32,
    }
    impl Card {
        fn new() -> Self {
            Card {
                winners: Vec::new(),
                current: Vec::new(),
                points: 0,
            }
        }
    }

    let inputs = read_input("inputs/day4.txt".to_string());
    let mut cards: Vec<Card> = Vec::new();
    for card in inputs {
        // for each card, create struct, find points.
        let (id, c) = card.split_once(": ").unwrap();
        let (win, got) = c.split_once("|").unwrap();

        let (_, id) = id.split_once(" ").unwrap();

        let mut cc = Card::new();
        let mut winners = Vec::new();
        let mut gotters = Vec::new();
        for w in win.split(" ") {
            match w.parse::<i32>() {
                Ok(v) => winners.push(v),
                Err(_) => {}
            }
        }

        for g in got.split(" ") {
            match g.parse::<i32>() {
                Ok(v) => gotters.push(v),
                Err(_) => {}
            }
        }

        // calculate points
        let mut points = 0;
        for g in &gotters {
            if winners.contains(&g) {
                points += 1
            }
        }
        cc.winners = winners;
        cc.current = gotters;
        cc.points = points;

        cards.push(cc);
    }

    // vec of size cards len with val 1 default
    dbg!(cards.len());
    let mut stack: Vec<i32> = vec![1; cards.len()];

    for index in 0..stack.len() {
        let cp = cards[index].points;
        // dbg!(cp, stack[index], index, (index + 1, cp + 1));
        for i in index + 1..(index as i32 + cp + 1) as usize {
            stack[i] += stack[index];
        }
    }

    dbg!(stack.iter().fold(0, |acc, x| acc + x));
}

fn day5() {
    let inputs = read_new("inputs/day5.txt");
    let mut sections: Vec<&str> = inputs.split("\n\n").collect();
    let mut maps: Vec<(&str, Vec<Vec<i64>>)> = Vec::new();
    let mut seeds: Vec<i64> = sections
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
        let mut curr_val = seed;
        for (name, mapping) in maps.iter() {
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

fn day6() {
    let inputs = read_input("inputs/day6sm.txt".to_string());
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

fn day7() {
    #[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug, Ord)]
    enum CardType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfaKind,
        FullHouse,
        FourOfaKind,
        FiveOfaKind,
    }

    #[derive(Debug, Eq, Ord)]
    struct Card<'a> {
        card_type: CardType,
        val: &'a str,
        bid: i32,
        ranks: [i32; 5],
    }
    impl PartialOrd for Card<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.card_type != other.card_type {
                return Some(self.card_type.cmp(&other.card_type));
            } else {
                // walk the ranks list
                for i in 0..5 {
                    if self.ranks[i] > other.ranks[i] {
                        return Some(Ordering::Greater);
                    } else if self.ranks[i] < other.ranks[i] {
                        return Some(Ordering::Less);
                    } else {
                        continue;
                    }
                }

                Some(Ordering::Equal)
            }
        }
    }

    impl PartialEq for Card<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.val == other.val
        }
    }

    fn get_ctype(val: &str) -> CardType {
        let mut counts: HashMap<char, i32> = HashMap::new();
        let c_type: CardType;
        let rank: Vec<i32> = Vec::new();

        for x in val.chars() {
            *counts.entry(x).or_default() += 1;
        }
        let x: Vec<i32> = counts.values().cloned().sorted().collect_vec();

        if x == vec![1, 1, 1, 1, 1] {
            CardType::HighCard
        } else if x == vec![1, 1, 1, 2] {
            CardType::OnePair
        } else if x == vec![1, 2, 2] {
            CardType::TwoPair
        } else if x == vec![1, 1, 3] {
            CardType::ThreeOfaKind
        } else if x == vec![2, 3] {
            CardType::FullHouse
        } else if x == vec![1, 4] {
            CardType::FourOfaKind
        } else {
            CardType::FiveOfaKind
        }
    }

    let inputs = read_input("inputs/day7.txt".to_string());
    let ranks = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let cards: Vec<(&str, &str)> = inputs.iter().map(|x| x.split_once(" ").unwrap()).collect();
    let mut c: Vec<Card> = Vec::new();
    for (val, bid) in cards {
        //figure  out card type
        // let counts =  v.iter().filter(|&n| *n == 91).count();

        let mut c_type: CardType;

        if val.contains('J') {
            //make new card

            let mut tmp_card = val;
            let mut all_ctypes: Vec<CardType> = Vec::new();
            let j_count = tmp_card.chars().filter(|&n| n == 'J').count();
            if j_count >= 4 {
                c_type = CardType::FiveOfaKind
            } else {
                let all_perms = itertools::repeat_n(ranks, j_count).multi_cartesian_product();
                let j_indices = tmp_card
                    .chars()
                    .enumerate()
                    .filter(|(_, r)| *r == 'J')
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();

                for perm in all_perms {
                    let mut inter_val = tmp_card.chars().collect_vec();
                    for (j_ind, p) in zip(&j_indices, perm) {
                        inter_val[*j_ind] = p;
                    }
                    let istr: String = inter_val.iter().collect();

                    all_ctypes.push(get_ctype(&istr));
                }

                all_ctypes.sort();
                all_ctypes.reverse();
                c_type = all_ctypes[0];
            }
        } else {
            c_type = get_ctype(val);
        }

        c.push(Card {
            card_type: c_type,
            val: val,
            bid: bid.parse().unwrap(),
            ranks: val
                .chars()
                .map(|x| ranks.iter().position(|&r| r == x).unwrap() as i32)
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap(),
        })
    }
    //sort by CardType
    c.sort();
    println!(
        "Answer: {}",
        c.iter()
            .enumerate()
            .map(|(idx, x)| x.bid * (idx + 1) as i32)
            .sum::<i32>()
    );
}

fn main() {
    // day1two();
    // day2();
    // day3();
    // day4();
    // day5();
    // day6();
    // day7();
}
