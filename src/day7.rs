use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
pub fn solve(inputs: Vec<String>) {
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

    let ranks = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let cards: Vec<(&str, &str)> = inputs.iter().map(|x| x.split_once(" ").unwrap()).collect();
    let mut c: Vec<Card> = Vec::new();
    for (val, bid) in cards {
        //figure  out card type
        // let counts =  v.iter().filter(|&n| *n == 91).count();

        let c_type: CardType;

        if val.contains('J') {
            //make new card

            let tmp_card = val;
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
