use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

#[allow(dead_code)]
fn remove_card(direction: &str, mut score: u16, deck1: &VecDeque<u8>, deck2: &VecDeque<u8>) -> u16 {

    let mut deck1 = deck1.clone();
    let mut deck2 = deck2.clone();

    if deck1[0] == deck2[0] {
        score += 1;
        deck1.pop_front();
        deck2.pop_front();
    }
    else if direction == "left" {
        deck1.pop_front();
    }
    else if direction == "right" {
        deck2.pop_front();
    }

    if deck1.len() == 0 || deck2.len() == 0 {
        return score
    }
    
    let opt1 = remove_card("left", score, &deck1, &deck2);
    let opt2 = remove_card("right", score, &deck1, &deck2);

    if opt1 > opt2 {
        return opt1;
    }
    opt2
}


#[allow(dead_code)]
fn remove_card_faster(mut already_seen: &mut HashMap<String, u16>, direction: &str, mut score: u16, deck1: &VecDeque<u64>, deck2: &VecDeque<u64>) -> u16 {

    let mut deck1: VecDeque<u64> = deck1.clone();
    let mut deck2: VecDeque<u64> = deck2.clone();

    if deck1[0] == deck2[0] {
        score += 1;
        deck1.pop_front();
        deck2.pop_front();
    }
    else if direction == "left" {
        deck1.pop_front();
    }
    else if direction == "right" {
        deck2.pop_front();
    }

    if deck1.len() == 0 || deck2.len() == 0 {
        return score
    }

    let seen_key: String = format!("{deck1:?}{deck2:?}");

    match already_seen.get(&seen_key) {
        Some(value) => return score + *value,
        None => (),
    }
    
    let opt1: u16 = remove_card_faster(&mut already_seen, "left", score, &deck1, &deck2);
    let opt2: u16 = remove_card_faster(&mut already_seen, "right", score, &deck1, &deck2);

    if opt1 > opt2 {
        let delta_score: u16 = opt1 - score;
        already_seen.insert(seen_key, delta_score);
        return opt1;
    }
    let delta_score: u16 = opt2 - score;
    already_seen.insert(seen_key, delta_score);
    opt2
}


pub fn run() {
    let stdin: io::Stdin = io::stdin();
    let mut iterator: io::Lines<io::StdinLock<'_>> = stdin.lock().lines();
    let _ = iterator.next().unwrap().unwrap();
    let line2: String = iterator.next().unwrap().unwrap();
    let line3: String = iterator.next().unwrap().unwrap();

    let line2: Vec<u64> = line2.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u64>>();
    let line3: Vec<u64> = line3.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u64>>();

    let deck1: VecDeque<u64> = VecDeque::from(line2);
    let deck2: VecDeque<u64> = VecDeque::from(line3);

    let mut already_seen: HashMap<String, u16> = HashMap::new();
    println!("{:?}", remove_card_faster(&mut already_seen, "start", 0, &deck1, &deck2));
}
