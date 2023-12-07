pub fn iflet_work(){

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // The if let syntax lets you combine if and let
    // It takes a pattern and an expression separated by an equal sign
    // It works the same way as a match
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin:: Nickel; 
    let count = value_in_cents3(coin);
    println!("The count is {}", count);

    let coin = Coin:: Quarter(UsState::Alabama); 
    let count = value_in_cents4(coin);
    println!("The count is {}", count);
}

#[derive(Debug)] 
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents4(coin: Coin) -> u8 {
    // We can include an else with an if let. 
    // The block of code that goes with the else is the same
    // as the block of code that would go with the _ case in the match expression
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    count
}

fn value_in_cents3(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
