#[path = "poker.rs"]
mod poker;

#[cfg(test)]
mod tests {

    use poker::poker::Card;
    use poker::poker::compare_hands;


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

    const HAND_FULL_HOUSE : [Card; 5] = [
        Card {
            suite: 'h',
            number: 3
        },Card {
            suite: 's',
            number: 3
        },Card {
            suite: 'h',
            number: 4
        },Card {
            suite: 's',
            number: 4
        },Card {
            suite: 'c',
            number: 4
        }
    ];

    const HAND_FOUR_OF_KIND : [Card; 5] = [
        Card {
            suite: 'h',
            number: 3
        },Card {
            suite: 's',
            number: 3
        },Card {
            suite: 'c',
            number: 3
        },Card {
            suite: 'd',
            number: 3
        },Card {
            suite: 'c',
            number: 4
        }
    ];
    #[test]
    fn strait_flush_beats_flush() {
        let mut hand_sf = HAND_STRAIT_FLUSH;
        let mut hand_f = HAND_FLUSH;
        assert_eq!(compare_hands(&mut hand_sf,&mut hand_f), 1);
    }
    #[test]
    fn flush_beats_strait() {
        let mut hand_s = HAND_STRAIT;
        let mut hand_f = HAND_FLUSH;
        assert_eq!(compare_hands(&mut hand_s,&mut hand_f), 2);
    }
    #[test]
    fn four_beats_strait() {
        let mut hand_s = HAND_STRAIT;
        let mut hand_f = HAND_FOUR_OF_KIND;
        assert_eq!(compare_hands(&mut hand_s,&mut hand_f), 2);
    }
    #[test]
    fn full_beats_flush() {
        let mut hand_flush = HAND_FLUSH;
        let mut hand_full = HAND_FULL_HOUSE;
        assert_eq!(compare_hands(&mut hand_full,&mut hand_flush), 1);
    }
}