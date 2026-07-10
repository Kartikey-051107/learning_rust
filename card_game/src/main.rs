#[derive(Debug)]
enum Suit{
    hearts,
    spades,
    diamonds,
    clubs,

}
#[derive(Debug)]
enum Rank{
    Number(u8),
    Ace,
    King,
    Queen,
    Jack,

}
#[derive(Debug)]
struct Card{
    suit:Suit,
    rank:Rank,

}
impl Card{
    fn new(suit:Suit,rank:Rank) ->Self{
        Self{ suit,rank}

        
    }
    fn value(&self)->u8{
        match self.rank{
            Rank::Number(n)=>n,
            Rank::Jack|Rank::Queen|Rank::King=>10,
            Rank::Ace=>11,

        }

    }
    fn display(&self){
        println!("{:?} of {:?} ",self.rank,self.suit)
    }
}
fn main() {
    let card1 = Card::new(Suit::hearts, Rank::Ace);
    let card2 = Card::new(Suit::spades, Rank::Number(7));
    let card3 = Card::new(Suit::clubs, Rank::King);

    card1.display();
    println!("Value = {}", card1.value());

    card2.display();
    println!("Value = {}", card2.value());

    card3.display();
    println!("Value = {}", card3.value());
}
