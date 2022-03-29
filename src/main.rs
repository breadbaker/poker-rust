// use std::io;


struct Card {
    suite: char,
    number: u8,
}

const HAND_FLUSH : [Card; 5] = [Card {
        suite: 'h',
        number: 3
    },Card {
        suite: 'h',
        number: 3
    },Card {
        suite: 'h',
        number: 3
    },Card {
        suite: 'h',
        number: 3
    },Card {
        suite: 'h',
        number: 3
    }
];


fn main() {


    println!("Guess the number!");

    println!("Please input your guess.");

    for i in 0..HAND_FLUSH.len() {
        println!("{}", HAND_FLUSH[i].number );
    }

    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("You guessed: {}", guess);
}