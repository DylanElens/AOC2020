use std::io;

fn compute(input: &mut Vec<String>) -> usize {
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut result: Vec<(&str, &str)> = Vec::new();
    for element in input {
        result.push(element.split_at(element.len() / 2));
    }

    let mut sum = 0;
    for element in result {
        for char in element.0.chars() {
            if element.1.contains(char) {
                sum += alphabet.find(char).unwrap() + 1;
                break;
            }
        }
    }

    return sum;
}

fn compute2(input: &mut Vec<String>) -> usize {
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut sum = 0;

    for element in input.chunks(3) {
        for item in element[0].chars(){
            if element[1].contains(item) && element[2].contains(item) {
                sum += alphabet.find(item).unwrap() + 1;
                break;
            }
        }
    }
    return sum;
}

fn main() {
    let mut input = Vec::new();
    io::stdin().lines().for_each(|line| {
        input.push(line.unwrap());
    });

    let result = compute(&mut input);
    let result2 = compute2(&mut input);
    println!("{}", result);
    println!("{}", result2);
}
