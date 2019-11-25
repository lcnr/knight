use tindex::bitset::TBitSet;

fn poly(n: i64, b: i64, c: i64) -> i64 {
    4 * n.pow(2) + b * n + c
}

fn pos_to_int((x, y): (i64, i64)) -> i64 {
    match (x >= 0, y >= 0) {
        (true, true) => {
            if x >= y {
                poly(x, -3, 1) + y
            } else {
                poly(y, -1, 1) - x
            }
        }
        (true, false) => {
            if x > y.abs() {
                poly(x, -3, 1) + y
            } else {
                poly(y, -3, 1) + x
            }
        }
        (false, true) => {
            if x.abs() >= y {
                poly(x, -1, 1) - y
            } else {
                poly(y, -1, 1) - x
            }
        }
        (false, false) => {
            if x.abs() >= y.abs() {
                poly(x, -1, 1) - y
            } else {
                poly(y, -3, 1) + x
            }
        }
    }
}

fn main() {
    let mut visited = TBitSet::new();
    let mut positions = Vec::new();
    
    visited.add(1);
    positions.push((0, 0));
    let mut x = 0;
    let mut y = 0;
    'outer: loop {
        let mut possible_next_steps = vec![
            (x + 2, y + 1),
            (x + 1, y + 2),
            (x - 1, y + 2),
            (x - 2, y + 1),
            (x - 2, y - 1),
            (x - 1, y - 2),
            (x + 1, y - 2),
            (x + 2, y - 1),
        ];

        possible_next_steps.sort_by_key(|&pos| pos_to_int(pos));
        for step in possible_next_steps {
            let pos = pos_to_int(step);
            if !visited.get(pos as usize) {
                visited.add(pos as usize);
                positions.push((x, y));
                x = step.0;
                y = step.1;
                continue 'outer;
            }
        }

        let prev = positions.pop().unwrap();
        println!(
            "stuck at ({}, {}) = {}, retreating to {:?} = {}",
            x,
            y,
            pos_to_int((x, y)),
            prev,
            pos_to_int(prev)
        );

        x = prev.0;
        y = prev.1;
    }
}
