pub fn solve(inputs: Vec<String>) {
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

    let mut cards: Vec<Card> = Vec::new();
    for card in inputs {
        // for each card, create struct, find points.
        let (id, c) = card.split_once(": ").unwrap();
        let (win, got) = c.split_once("|").unwrap();

        let (_, _) = id.split_once(" ").unwrap();

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
