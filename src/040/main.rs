use std::time::Instant;

fn generate_champernownes_constant(size: usize) -> String {
    let mut next = 1;
    let mut cons = "".to_string();
    while cons.len() <= size {
        cons = cons + (&next.to_string());
        next += 1;
    }
    cons
}

fn main() {
    let now = Instant::now();

    let cons = generate_champernownes_constant(1_000_000);
    let one = cons.clone().chars().nth(1 - 1).unwrap() as usize - 0x30;
    let ten = cons.clone().chars().nth(10 - 1).unwrap() as usize - 0x30;
    let hun = cons.clone().chars().nth(100 - 1).unwrap() as usize - 0x30;
    let tho = cons.clone().chars().nth(1000 - 1).unwrap() as usize - 0x30;
    let tth = cons.clone().chars().nth(10_000 - 1).unwrap() as usize - 0x30;
    let hth = cons.clone().chars().nth(100_000 - 1).unwrap() as usize - 0x30;
    let mil = cons.clone().chars().nth(1_000_000 - 1).unwrap() as usize - 0x30;
    println!("{}", one * ten * hun * tho * tth * hth * mil);
    let elapsed = now.elapsed();
    println!("Took {} miliseconds", elapsed.as_millis());
}
