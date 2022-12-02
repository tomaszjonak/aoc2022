use std::io;

fn main() -> io::Result<()> {
    let mut max = 0;
    let mut acc = 0;
    for line in io::stdin().lines() {
        let Ok(line) = line else { continue };
        if line.len() == 0 {
            max = std::cmp::max(max, acc);
            acc = 0;
        }
        let Ok(val) = line.parse::<u32>() else { continue };
        acc += val;
    }
    max = std::cmp::max(max, acc);
    println!("{}", max);
    Ok(())
}
