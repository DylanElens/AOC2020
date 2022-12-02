use std::io;

fn compute(input: &mut Vec<String>) -> i32 {
    let mut list: Vec::<i32> = Vec::new();
    let mut sum = 0;
    for element in input {
        if element != "" {
            let i: i32 = element.parse().unwrap();
            sum += i;
        } else {
            list.push(sum);
            sum = 0;
        }
    }

    list.sort();
    return list[list.len() - 1] + list[list.len() - 2] + list[list.len() - 3];
}

fn main() {
    let mut input = Vec::new();
    io::stdin().lines().for_each(|line| {
        input.push(line.unwrap());
    });

    let result = compute(&mut input);
    println!("{}", result);
    
    
}
