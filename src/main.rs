// let config_max = Some(3u8);
// }

let config_max = Some(3u8);

// match config_max {
//     Some(max) => println!("The max is {}", max),
//     _ => (),
// by using if let we can simplify the code
if let Some(max) = config_max {
    println!("The max is {}", max);
} // you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values

// let mut count = 0;
// match coin {
//     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//     _ => count += 1,
// }

// an if let w/ an else is like a match that runs code when the value matches or else runs some other code
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}