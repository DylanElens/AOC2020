use std::io;

fn compute(input: &mut Vec<String>) -> usize {
    let mut range_fully_contains = 0;
    for element in input {
        let tuple = element.split(",").collect::<Vec<&str>>();
        let x = tuple[0].split("-").collect::<Vec<&str>>();
        let y = tuple[1].split("-").collect::<Vec<&str>>();
        let range1start = x[0].parse::<usize>().unwrap();
        let range1stop = x[1].parse::<usize>().unwrap();
        let range2start = y[0].parse::<usize>().unwrap();
        let range2stop = y[1].parse::<usize>().unwrap();

        if range1start <= range2start && range2stop <= range1stop {
            range_fully_contains += 1;
        }
        else if range2start <= range1start && range1stop <= range2stop {
            range_fully_contains += 1;
        }
    }

    return range_fully_contains;
}

fn compute2(input: &mut Vec<String>) -> usize {
    let mut overlaps = 0;
    for element in input {
        let tuple = element.split(",").collect::<Vec<&str>>();
        let x = tuple[0].split("-").collect::<Vec<&str>>();
        let y = tuple[1].split("-").collect::<Vec<&str>>();
        let range1start = x[0].parse::<usize>().unwrap();
        let range1stop = x[1].parse::<usize>().unwrap();
        let range2start = y[0].parse::<usize>().unwrap();
        let range2stop = y[1].parse::<usize>().unwrap();

        if range1start <= range2start && range2start <= range1stop {
            overlaps += 1;
        }
        else if range2start <= range1start && range1start <= range2stop {
            overlaps += 1;
        }


    }
    return overlaps;
}

fn main() {
    let mut input = Vec::new();
    io::stdin().lines().for_each(|line| {
        input.push(line.unwrap());
    });

    let result = compute(&mut input);
    let result2 = compute2(&mut input);
    println!("{}", input.len());
    println!("{}", result);
    println!("{}", result2);
}
