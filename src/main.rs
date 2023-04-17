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
    println!("{:?}", card1);
    println!("{:?}", card2);
}
