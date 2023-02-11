
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;
use std::io;

//baccarat rules from https://www.caesars.com/casino-gaming-blog/latest-posts/table-games/baccarat/how-to-play-baccarat
fn main() {
    let mut player_wins = 0;
    let mut dealer_wins = 0;
    let mut ties = 0;
    let mut tie_streak = 0;
    let mut player_streak = 0;
    let mut dealer_streak = 0;
    let mut highest_tie = 0;
    let mut highest_player = 0;
    let mut highest_dealer = 0;
    let mut last_win = String::new();
    let mut shoes = String::new();
    println!("How many shoes would you like to simulate?");
    io::stdin().read_line(&mut shoes).expect("Failed");
    let mut num_shoes = shoes.trim().parse().expect("Not int");
    for n in 0..num_shoes {
        //create the shoe to use for play
        //this is messy 
        let mut shoe: Vec<u8> = Vec::new();
        for _ in 0..8 {
            let deck = create_deck();
            shoe.extend(deck);
        }
        let mut deck = shoe.clone();
        let cut_card: usize = thread_rng().gen_range(270..354);
        // play the hands in the shoe
        while deck.len() > (cut_card){
            let tup = play_hand(deck);
            deck = tup.0;
            let mut player: Vec<u8> = tup.1;
            let mut dealer: Vec<u8> = tup.2;
            if check_outcome(&player, &dealer) == "Tie".to_string(){
                ties += 1;
                if last_win == "Tie".to_string() {
                    tie_streak += 1;
                    if highest_tie < tie_streak {
                        highest_tie = tie_streak;
                    }
                }
                last_win = "Tie".to_string();
                dealer_streak = 0;
                player_streak = 0;
            }
            else if check_outcome(&player, &dealer) == "Player".to_string() {
                player_wins += 1;
                if last_win == "Player".to_string() {
                    player_streak += 1;
                    if highest_player < player_streak {
                        highest_player = player_streak;
                    }
                }
                last_win = "Player".to_string();
                dealer_streak = 0;
                tie_streak = 0;
            }
            else {
                dealer_wins += 1;
                if last_win == "Dealer".to_string() {
                    dealer_streak += 1;
                    if highest_dealer < dealer_streak {
                        highest_dealer = dealer_streak;
                    }
                }
                last_win = "Dealer".to_string();
                player_streak = 0;
                tie_streak = 0;
            }
        }
        println!("Player Wins: {} Dealer Wins: {} Ties: {} Shoe Number: {}", player_wins, dealer_wins, ties, n);
    }
    println!("Player Wins %: {:?} Dealer Wins %: {:?} Tie % {:?}", ((player_wins as f32/(player_wins+dealer_wins+ties) as f32) *100.00), ((dealer_wins as f32 /(player_wins+dealer_wins+ties) as f32) * 100.00), ((ties as f32/(player_wins+dealer_wins+ties) as f32) * 100.00));
    println!("Highest streak - Player: {} Dealer: {} Tie: {}", highest_player, highest_dealer, highest_tie);
}

//convert to hand to card representation
fn hand_as_cards(hand: &Vec<u8>) -> String {
    let mut s = String::new();
    for card in hand.iter() {
        s.push_str(&get_card(card)); 
    }
    s
}

//convert card from int to string
fn get_card(card: &u8) -> String {
    match card {
        0 => "2S".to_string(),
        1 => "3S".to_string(),
        2 => "4S".to_string(),
        3 => "5S".to_string(),
        4 => "6S".to_string(),
        5 => "7S".to_string(),
        6 => "8S".to_string(),
        7 => "9S".to_string(),
        8 => "10S".to_string(),
        9 => "JS".to_string(),
        10 => "QS".to_string(),
        11 => "KS".to_string(),
        12 => "AS".to_string(),
        13 => "2C".to_string(),
        14 => "3C".to_string(),
        15 => "4C".to_string(),
        16 => "5C".to_string(),
        17 => "6C".to_string(),
        18 => "7C".to_string(),
        19 => "8C".to_string(),
        20 => "9C".to_string(),
        21 => "10C".to_string(),
        22 => "JC".to_string(),
        23 => "QC".to_string(),
        24 => "KC".to_string(),
        25 => "AC".to_string(),
        26 => "2H".to_string(),
        27 => "3H".to_string(),
        28 => "4H".to_string(),
        29 => "5H".to_string(),
        30 => "6H".to_string(),
        31 => "7H".to_string(),
        32 => "8H".to_string(),
        33 => "9H".to_string(),
        34 => "10H".to_string(),
        35 => "JH".to_string(),
        36 => "QH".to_string(),
        37 => "KH".to_string(),
        38 => "AH".to_string(),
        39 => "2D".to_string(),
        40 => "3D".to_string(),
        41 => "4D".to_string(),
        42 => "5D".to_string(),
        43 => "6D".to_string(),
        44 => "7D".to_string(),
        45 => "8D".to_string(),
        46 => "9D".to_string(),
        47 => "10D".to_string(),
        48 => "JD".to_string(),
        49 => "QD".to_string(),
        50 => "KD".to_string(),
        51 => "AD".to_string(),
        _ => "Error".to_string(),
    }
}

//convert card to its value in baccarat
// face cards = 10 Ace = 1
fn card_value(card: &u8) -> u8 {
    match card {
        0 => 2,
        1 => 3,
        2 => 4,
        3 => 5,
        4 => 6,
        5 => 7,
        6 => 8,
        7 => 9,
        8 => 10,
        9 => 10,
        10 => 10,
        11 => 10,
        12 => 1,
        13 => 2,
        14 => 3,
        15 => 4,
        16 => 5,
        17 => 6,
        18 => 7,
        19 => 8,
        20 => 9,
        21 => 10,
        22 => 10,
        23 => 10,
        24 => 10,
        25 => 1,
        26 => 2,
        27 => 3,
        28 => 4,
        29 => 5,
        30 => 6,
        31 => 7,
        32 => 8,
        33 => 9,
        34 => 10,
        35 => 10,
        36 => 10,
        37 => 10,
        38 => 1,
        39 => 2,
        40 => 3,
        41 => 4,
        42 => 5,
        43 => 6,
        44 => 7,
        45 => 8,
        46 => 9,
        47 => 10,
        48 => 10,
        49 => 10,
        50 => 10,
        51 => 1,
        _ => 0, 
    }
}

// get the baccarat hand value
fn hand_value(hand: &Vec<u8>) -> u16 {
    let mut value: u16 = 0;
    for i in hand {
        value += card_value(i) as u16;
    }
    value % 10
}

//create a shuffled deck
fn create_deck() -> Vec<u8> {
    let unshuffled_deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51];
    let mut shuffled_deck = Vec::new();
    let mut rng = rand::thread_rng();
    shuffled_deck = unshuffled_deck.clone();
    shuffled_deck.shuffle(&mut rng);
    shuffled_deck
}

//play through 1 hand and return the deck and hands
fn play_hand(mut deck: Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {

    let mut player = Vec::new();
    let mut dealer = Vec::new();
    let mut player_score:u8 = 0;
    let mut dealer_score:u8 = 0;
    let mut deck_index: u8 = 3;

    player.push(deck[0]);
    dealer.push(deck[1]);
    player.push(deck[2]);
    dealer.push(deck[3]);

    if hand_value(&player) > 7 || hand_value(&dealer) > 7 {
        deck.drain(0..(deck_index as usize));
        return (deck, player, dealer);
    }

    while hand_value(&player) < 6{
        player.push(deck[(deck_index+1) as usize ]);
        deck_index += 1;
    }

    while hand_value(&dealer) < 6{
        dealer.push(deck[(deck_index+1) as usize]);
        deck_index += 1;
    }
    deck.drain(0..(deck_index as usize));
    (deck, player, dealer)

}

//check for tie or winner of the hand
fn check_outcome(player:&Vec<u8>, dealer:&Vec<u8>) -> String {
    if hand_value(&player) == hand_value(&dealer){
        "Tie".to_string()
    }
    else if hand_value(&player) > hand_value(&dealer){
        "Player".to_string()
    }

    else {
        "Dealer".to_string()
    }

}