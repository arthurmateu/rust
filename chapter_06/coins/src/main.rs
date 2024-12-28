fn main() {
    let purse = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    let mut total = 0;
    for money in purse {
        total = total + value_in_cents(money)
    }
    println!("I have $0.{}", total);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Invalid,
    Special,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => {
            println!("I don't think this one's valid...?");
            0
        }
    }

    if let Coin::Special = special_coin {
        println!("Ooh, shiny!");
    }
}
