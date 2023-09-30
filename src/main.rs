use std::io;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Board {
    turn: usize,
    players: [Player; 2],
    current_player: usize,
}

#[derive(Debug,Clone)]
struct Player {
    life: usize,
    turn: usize,
    mana: usize,
    domain: Domain,
}

#[derive(Debug,Clone)]
struct Domain {
    library: Vec<Card>,
    hand: Vec<Card>,
    battlefield: Vec<Card>,
    graveyard: Vec<Card>,
}

#[derive(Debug,Clone)]
struct Card {
    id: usize,
    card_text: CardText
}

#[derive(Debug,Clone)]
struct CardText {
    name: String,
    cost: usize,
    power: usize,
    toughness: usize,
}

#[derive(Debug, Clone)]
struct Action {
    requirement: Requirement,
    effect: Effect,
}

#[derive(Debug, Clone)]
struct Requirement {
    cost: usize,
}

#[derive(Debug, Clone)]
enum Effect {
    Draw,
    Damage,
    Movement,
}

impl Board {
    fn cleanup(&mut self) {
        self.turn = self.turn + 1;
        self.current_player = 1 - self.current_player;
    }
    fn render(&mut self) {
        println!("*** TURN {}, PLAYER {} ***", self.turn, self.current_player);
        println!("Opponent's Battlefield:");
        for (i,value) in self.players[1-self.current_player].domain.battlefield.iter().enumerate() {
            println!("    Index {}: <<{}>>", i, value.card_text.name);
        }
        println!("Your Battlefield:");
        for (i,value) in self.players[self.current_player].domain.battlefield.iter().enumerate() {
            println!("    Index {}: <<{}>>", i, value.card_text.name);
        }
        println!("Your hand:");
        for (i,value) in self.players[self.current_player].domain.hand.iter().enumerate() {
            println!("    Index {}: ({}) <<{}>>", i, value.card_text.cost, value.card_text.name);
        }
        println!("Your mana: {}/{}", self.players[self.current_player].mana, self.players[self.current_player].turn);
        println!("************************");
    }
}

impl Player {
    fn upkeep(&mut self) {
        self.draw();
        self.mana = self.turn;
    }
    fn draw(&mut self) {
        let length = self.domain.library.len();
        if length>0 {
            let card = self.domain.library.remove(0);
            self.domain.hand.push(card);    
        } else {
            println!("No card in your library.")
        }
    }
    fn cast(&mut self, hand_index: usize) {
        let card = self.domain.hand.remove(hand_index);
        self.mana = self.mana - card.card_text.cost;
        self.domain.battlefield.push(card);
    }
    // fn draw_cards(&mut self, n: usize) {
    //     let max_n=self.domain.library.len().try_into().unwrap();
    //     if 0<n && n<=max_n {
    //         for i in 1..=n {
    //             let card = self.domain.library.remove(0);
    //             self.domain.hand.push(card);
    //         }
    //     } else if n>max_n {
    //         for i in 1..=max_n {
    //             let card = self.domain.library.remove(0);
    //             self.domain.hand.push(card);
    //         }
    //     } else {
    //         ();
    //     }
    // }
}

trait Agent {
    fn get_cast_num(&self, board: &Board) -> usize;
    // fn draw_a_card(&self);
    // fn draw(&mut self, n: usize) {
    //     let max_n=self.domain.library.len().try_into().unwrap();
    //     if 0<n && n<=max_n {
    //         for i in 1..=n {
    //             let card = self.domain.library.remove(0);
    //             self.domain.hand.push(card);
    //         }
    //     } else if n>max_n {
    //         for i in 1..=max_n {
    //             let card = self.domain.library.remove(0);
    //             self.domain.hand.push(card);
    //         }
    //     } else {
    //         ();
    //     }
    // }
}

#[derive(Debug, Clone, Copy)]
struct Human;
impl Agent for Human {
    fn get_cast_num(&self, board: &Board) -> usize {
        let mut input = String::new();
        println!("Choose a index to cast");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let hand_number: usize = input
            .trim()
            .parse()
            .expect("Failed to parse input as an usize");
        hand_number
    }
}

#[derive(Debug, Clone, Copy)]
struct RandomAgent;
impl Agent for RandomAgent {
    fn get_cast_num(&self, board: &Board) -> usize {
        let mut rng = rand::thread_rng();
        let random_usize = rng.gen_range(0..3);
        random_usize
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
        let j:usize = i/3 + 1;
        let c = Card {
            id: i,
            card_text: CardText {
                name: format!("Sample Card {} A", j),
                cost: j,
                power: j,
                toughness: j,
            }
        };
        let d = Card {
            id: i,
            card_text: CardText {
                name: format!("Sample Card {} B", j),
                cost: j+1,
                power: j,
                toughness: j,
            }
        };
        player1_deck.push(c);
        player2_deck.push(d);
    }
    let mut rng = thread_rng();
    player1_deck.shuffle(&mut rng);
    let player1 = Player {
        life: 13,
        turn: 3,
        mana: 5,
        domain: Domain {
            library: player1_deck,
            hand: vec![],
            battlefield: vec![],
            graveyard: vec![],
        },
    };
    let turn: usize = 0;
    let mut current_player: usize = 0;
    let agent0 = RandomAgent {};
    let agent1 = Human {};
    let agents: [Box<dyn Agent>; 2] = [Box::new(agent0), Box::new(agent1)];
    let mut board = Board {
        turn: turn,
        players: [player1.clone(), player1.clone()],
        current_player: 0,
    };
    // println!("{:?}", board);
    loop {
        board.players[board.current_player].upkeep();
        board.render();
        println!("{:?}", board.turn);
        println!("{:?}", board.current_player);
        for _i in 0..2 {
            board.players[board.current_player].draw();
        }
        let current_agent = &agents[board.current_player];
        // println!("Player {} mana: {:?}", board.current_player, board.players[board.current_player].mana);
        // println!("{:?}", board.players[board.current_player].domain.hand);
        let hand_index = current_agent.get_cast_num(&board);
        // println!("Cast hand-index {:?}", &hand_index);
        board.players[board.current_player].cast(hand_index);
        // println!("{:?}", board.players[board.current_player].domain.battlefield);
        // println!("Player {} mana: {:?}", board.current_player, board.players[board.current_player].mana);
        // println!("{:?}", board.players[board.current_player].domain.hand);
        board.cleanup();
        if board.turn>10 {
            break;
        }
    }
}
