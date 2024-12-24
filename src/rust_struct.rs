// // mod guessing_game;
// // mod temperature_converter;
// mod first_word;

// // use guessing_game::play_guessing_game;
// // use temperature_converter::temperature_converter_cel_to_fah;
// use first_word::get_first_word;



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
    Alabama,
    Alaska,
    // --snip--
    }

pub fn structs() {
  
    let some_u8_value = Some(0u8);
    match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
    }

    if let Some(0u8) = some_u8_value{
        println!("hahahh")
    }

}

