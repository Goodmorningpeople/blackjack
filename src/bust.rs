// returns if card total is over 21
pub fn is_bust(cards: &Vec<i32>) -> bool {
    let mut sum = 0;
    for card in cards {
        sum += card;
    }
    if sum > 21 {
        return true;
    }
    false
}
