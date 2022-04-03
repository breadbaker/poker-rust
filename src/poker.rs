pub mod poker {
    use std::collections::HashMap;

    #[derive(Clone, Copy, Debug)]
    pub struct Card {
        pub suite: char,
        pub number: u8,
    }

    fn is_flush(hand: [Card; 5]) -> bool {
        for i in 1..hand.len() {
            if hand[i].suite != hand[0].suite {
                return false
            }
        }
        true
    }
    
    fn number_freq_vector(hand: [Card; 5])  -> Vec<u8> {
        let mut number_freq = HashMap::new();
        let starter: u8 = 0;
        let incremetor: u8 = 1;
        for i in 0..hand.len() {
            *number_freq.entry(hand[i].number).or_insert(starter) += incremetor;
        }
    
        let mut freq_vector_sorted = Vec::new();
    
        for  (_k, v) in number_freq.iter() {
            freq_vector_sorted.push(*v);
        }
    
        freq_vector_sorted.sort();
        freq_vector_sorted
    }
    
    fn is_strait(hand: [Card; 5]) -> bool {
        for i in 1..hand.len() {
            if hand[i].number != hand[i - 1].number + 1 {
                return false
            }
        }
        true
    }
    
    fn is_full_house(hand: [Card; 5]) -> bool {
        number_freq_vector(hand) == [2,3]
    }
    
    fn is_four_of_kind(hand: [Card; 5]) -> bool {
        number_freq_vector(hand) == [1,4]
    }
    
    fn is_three_of_kind(hand: [Card; 5]) -> bool {
        number_freq_vector(hand) == [1,1,3]
    }
    
    fn is_two_pair(hand: [Card; 5]) -> bool {
        number_freq_vector(hand) == [1,2,2]
    }
    
    fn is_pair(hand: [Card; 5]) -> bool {
        number_freq_vector(hand) == [1,1,1,2]
    }
    
    fn is_high_card(_hand: [Card; 5]) -> bool {
        true
    }
    
    fn is_strait_flush(hand: [Card; 5]) -> bool {
        is_strait(hand) && is_flush(hand)
    }
    
    fn sort_hand( hand: &mut [Card; 5]) {
        hand.sort_by_key(|card| card.number);
    }

    pub fn compare_hands(hand_one: &mut [Card; 5], hand_two: &mut [Card; 5]) -> u8 {
        println!("Poker Compare Hands!");
        println!("Hand one: {:?}", hand_one);
        println!("Hand two: {:?}", hand_two);
        sort_hand(hand_one);
        sort_hand(hand_two);

        const HAND_FNS : [fn([Card; 5]) -> bool; 9] = [
            is_strait_flush,
            is_four_of_kind,
            is_full_house,
            is_flush,
            is_strait,
            is_three_of_kind,
            is_two_pair,
            is_pair,
            is_high_card
        ];

        for i in 0..HAND_FNS.len() {
            let hand_one_result :bool = HAND_FNS[i](*hand_one);
            let hand_two_result :bool = HAND_FNS[i](*hand_two);
            if hand_one_result && hand_two_result {
                println!("its a tie {}", i );
                return 3;
            }
            if !hand_one_result && hand_two_result {
                println!("hand two {}", i );
                return 2;
            }
            if hand_one_result && !hand_two_result {
                println!("hand one {}", i );
                return 1;
            }
        }
        return 0;
    }
}