use regex::Regex;

pub fn solve(inputs: Vec<String>) {
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
