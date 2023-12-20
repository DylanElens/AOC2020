use std::io;

#[derive(Debug)]
struct Command {
    amount: u32,
    from: u32,
    to: u32,
}

fn compute(game: &mut Vec<String>, commands: &mut Vec<String>) -> usize {

    commands.into_iter() // Convert the Vec into an iterator
    .map(|s| {
        s.split_whitespace() // Split the string into an iterator
        .map(|s| s.parse::<u32>()) // Convert the iterator into a Vec
        .collect::<Result<Vec<u32>, _>>() // Convert the Result into a Vec
        .into_iter()
        .fold(Command { amount: 0, from: 0, to: 0 }, |mut acc, x| {
            match acc.amount {
                0 => acc.amount = x,
                1 => acc.from = x,
                2 => acc.to = x,
                _ => (),
            }
            acc
        })
    })
    .collect(); // Collect the Move structs into a Vec
    return 0;
}
//
// fn compute2(input: &mut Vec<String>) -> usize {
// }

fn main() {
    let mut game = Vec::new();
    let mut commands = Vec::new();
    io::stdin().lines().for_each(|line| {
        let actual_line = line.unwrap();
        if actual_line.contains("move"){
            commands.push(actual_line);
        } else {
            game.push(actual_line);
        }
    });
    
    println!("game: {:?}", game);
    println!("commands: {:?}", commands);
    let result = compute(&mut game, &mut commands);
}
