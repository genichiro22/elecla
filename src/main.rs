use std::io;
use rand::Rng;

#[derive(Debug)]
struct Board {
    turn: u32,
    player1: Player,
    player2: Player,
}

#[derive(Debug)]
struct Player {
    life: u32,
    turn: u32,
    mana: u32,
    domain: Domain,
}

#[derive(Debug)]
struct Domain {
    library: Vec<Card>,
    hand: Vec<Card>,
    field: Vec<Card>,
    graveyard: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    id: u32,
    card_text: CardText
}

#[derive(Debug, Clone)]
struct CardText {
    name: String,
    cost: u32,
    power: u32,
    toughness: u32,
}

impl Player {
    fn draw(&mut self, n: u32) {
        let max_n=self.domain.library.len().try_into().unwrap();
        if 0<n && n<=max_n {
            for i in 1..=n {
                let card = self.domain.library.remove(0);
                self.domain.hand.push(card);
            }
        } else if n>max_n {
            for i in 1..=max_n {
                let card = self.domain.library.remove(0);
                self.domain.hand.push(card);
            }
        } else {
            ();
        }
    }
}

trait Agent {
    fn get_cast_num(&self) -> u32;
}

struct Human {
    player: u32,
}

impl Agent for Human {
    fn get_cast_num(&self) -> u32 {
        let mut input = String::new();
        println!("Choose a number to cast");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let hand_number: u32 = input
            .trim()
            .parse()
            .expect("Failed to parse input as an u32");
        hand_number
    }
}

#[derive(Debug,Clone,Copy)]
struct RandomAgent {
    player: u32,
}
impl Agent for RandomAgent {
    fn get_cast_num(&self) -> u32 {
        let mut rng = rand::thread_rng();
        let random_u32 = rng.gen::<u32>();
        random_u32
    }
}

fn main() {
    let llmc = CardText {
        name: String::from("Lhollmach of the Strength"),
        cost: 4,
        power: 6,
        toughness: 6,
    };
    let card1 = Card {
        id: 1,
        card_text: llmc.clone(),
    };
    let card2 = Card {
        id: 2,
        card_text: llmc.clone(),
    };
    let mut player1_deck: Vec<Card> = Vec::new();
    let mut player2_deck: Vec<Card> = Vec::new();
    for i in 1..=30 {
        let j:u32 = i/3 + 1;
        let c = Card {
            id: i,
            card_text: CardText {
                name: format!("Sample {}", j),
                cost: j,
                power: j,
                toughness: j,
            }
        };
        let d = Card {
            id: i,
            card_text: CardText {
                name: format!("PL2 {}", j),
                cost: j+1,
                power: j,
                toughness: j,
            }
        };
        player1_deck.push(c);
        player2_deck.push(d);
    }
    let mut player1 = Player {
        life: 13,
        turn: 3,
        mana: 5,
        domain: Domain {
            library: player1_deck,
            hand: vec![],
            field: vec![],
            graveyard: vec![],
        },
    };
    player1.draw(1);
    let mut turn: u32 = 0;
    let mut current_player: u32 = 1;
    let agent1 = RandomAgent {
        player: 1,
    };
    let agent2 = RandomAgent {
        player: 2,
    };

    loop {
        current_player = 2 - current_player;
        let agent = match current_player {
            1 => agent1,
            2 => agent2,
            _ => panic!("Invalid value encountered!"),
        };
        turn = turn + 1;
        if turn>10 {
            break;
        }
    }
}
