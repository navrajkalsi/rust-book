#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            // match forces you to handle every possible enum value
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // enum variants can have different type associated with them
}

fn describe_state_quarter_if_let(coin: Coin) -> Option<String> {
    // one way to write this
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("if let: {state:?} is old"))
        } else {
            Some(format!("if let: {state:?} is new"))
        }
    } else {
        None
    }
}

fn describe_state_quarter_if(coin: Coin) -> Option<String> {
    // split the state retrieval and returns
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("if: {state:?} is old"))
    } else {
        Some(format!("if: {state:?} is new"))
    }
}

fn describe_state_quarter_let(coin: Coin) -> Option<String> {
    // this just checks the coin is a quarter and returns none if not
    // this also defines state in the current scope
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("let: {state:?} is old"))
    } else {
        Some(format!("let: {state:?} is new"))
    }
}

fn main() {
    if let Some(desc) = describe_state_quarter_if_let(Coin::Quarter(UsState::Alaska)) {
        // if let does not force you to handle every possible return value, not handling None here
        println!("{desc}");
    }
    if let Some(desc) = describe_state_quarter_if(Coin::Quarter(UsState::Alaska)) {
        // if let does not force you to handle every possible return value, not handling None here
        println!("{desc}");
    }
    if let Some(desc) = describe_state_quarter_let(Coin::Quarter(UsState::Alaska)) {
        // if let does not force you to handle every possible return value, not handling None here
        println!("{desc}");
    }
}
