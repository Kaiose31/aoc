use std::collections::HashMap;

pub fn solve(input: Vec<String>) {
    #[derive(Debug, Clone, Copy)]
    struct Game {
        id: u32,
        red: u32,
        blue: u32,
        green: u32,
    }
    impl Game {
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
