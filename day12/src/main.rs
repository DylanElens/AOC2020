use std::io;

#[derive(PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn is_valid_move(board: &Vec<Vec<String>>, current_pos: &Point, target_pos: &Point) -> bool {
    let alphabet = "SabcdefghijklmnopqrstuvwxyEzE";
    let alphabet = alphabet.chars().collect::<Vec<char>>();
    let current_pos = &board[current_pos.y as usize][current_pos.x as usize];
    let current_pos_val = alphabet
        .iter()
        .position(|&x| x == current_pos.chars().nth(0).unwrap())
        .unwrap();
    let target_pos = &board[target_pos.y as usize][target_pos.x as usize];
    let target_pos_val = alphabet
        .iter()
        .position(|&x| x == target_pos.chars().nth(0).unwrap())
        .unwrap();

    if current_pos_val + 1 == target_pos_val || current_pos_val - 1 == target_pos_val {
        return true;
    } else {
        return false;
    }
}

fn compute(board: &mut Vec<Vec<String>>) -> usize {
    let mut queue = Vec::new();
    let mut visited = Vec::new();
    let mut steps = 0;
    let mut x = 0;
    let mut y = 0;
    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == "S" {
                x = i;
                y = j;
            }
        }
    }
    queue.push((x, y, steps));

    while !queue.is_empty() {
        let (x, y, steps) = queue.remove(0);
        let current_pos = Point { x, y };
        let neighbours = vec![
            Point { x: x + 1, y: y },
            Point { x: x - 1, y: y },
            Point { x: x, y: y + 1 },
            Point { x: x, y: y - 1 },
        ];
        let reachable_neightbours = neighbours.iter().filter(|point| {
            return point.x < board.len()
                && point.y < board[0].len()
                && is_valid_move(&board, &current_pos, &point)
                && !visited.contains(&point);
        });
        
        for neighbour in reachable_neightbours {
            if board[neighbour.x][neighbour.y] == "E" {
                return steps + 1;
            }
            queue.push((neighbour.x, neighbour.y, steps + 1));
            visited.push(&neighbour);
        }
        // if board[x].iter().enumerate().nth(y).unwrap().1 == "E" {
        //     return steps;
        // }

    }
    return 0;
}

fn main() {
    let mut game = Vec::new();
    io::stdin().lines().for_each(|line| {
        let mut row = Vec::new();
        for c in line.unwrap().chars() {
            row.push(String::from(c));
        }
        game.push(row);
    });

    let result = compute(&mut game);
    println!("result: {:?}", result);
}
