// use std::io;

// mermaids reef

// calypso

// colors

// cherokee

// bahama palm

// blue holes  - hard to find

// hope town inn and marina



#[derive(Clone, Copy, Debug)]
struct Card {
    suite: char,
    number: u8,
}

const HAND_STRAIT_FLUSH : [Card; 5] = [
    Card {
        suite: 'h',
        number: 1
    },Card {
        suite: 'h',
        number: 2
    },Card {
        suite: 'h',
        number: 5
    },Card {
        suite: 'h',
        number: 4
    },Card {
        suite: 'h',
        number: 3
    }
];

const HAND_STRAIT : [Card; 5] = [
    Card {
        suite: 'c',
        number: 1
    },Card {
        suite: 'h',
        number: 2
    },Card {
        suite: 'h',
        number: 5
    },Card {
        suite: 'h',
        number: 4
    },Card {
        suite: 'h',
        number: 3
    }
];

const HAND_FLUSH : [Card; 5] = [
    Card {
        suite: 'h',
        number: 3
    },Card {
        suite: 'h',
        number: 4
    },Card {
        suite: 'h',
        number: 5
    },Card {
        suite: 'h',
        number: 6
    },Card {
        suite: 'h',
        number: 9
    }
];


fn is_flush(hand: [Card; 5]) -> bool {
    for i in 1..hand.len() {
        if hand[i].suite != hand[0].suite {
            return false
        }
    }
    true
}

// fn number_freq_vector(hand: [Card; 5]) -> Vec {

//     for i in 1..hand.len() {
//         if hand[i].number != hand[i - 1].number + 1 {
//             return false
//         }
//     }
//     let sum = v.into_iter().reduce(|a, b| a + b);
// }

fn is_strait(hand: [Card; 5]) -> bool {
    for i in 1..hand.len() {
        if hand[i].number != hand[i - 1].number + 1 {
            return false
        }
    }
    true
}

fn sort_hand( hand: &mut [Card; 5]) {
    hand.sort_by_key(|card| card.number);
}


fn main() {


    println!("Guess the number!");

    println!("Please input your guess.");

    let mut hand_one = HAND_STRAIT;

    let mut hand_two = HAND_FLUSH;

    sort_hand(&mut hand_one);
    sort_hand(&mut hand_two);



    for i in 0..hand_two.len() {
        println!("{:?}", hand_two[i] );
        // println!("{}", hand_two[i].number );
    }

    println!("is hand_one flush{}", is_flush(hand_one));
    println!("is hand_one strait{}", is_strait(hand_one));
    println!("is hand_two flush{}", is_flush(hand_two));
    println!("is hand_two strait{}", is_strait(hand_two));
    // println!("hand_sorted {}", sort_hand(hand_two)[0].suite);

    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("You guessed: {}", guess);
}