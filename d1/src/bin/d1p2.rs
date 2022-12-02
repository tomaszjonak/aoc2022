use std::io;

fn main() -> io::Result<()> {
    let mut sums = Vec::new();
    let mut acc = 0;
    for line in io::stdin().lines() {
        let Ok(line) = line else { continue };
        if line.len() == 0 {
            sums.push(acc);
            acc = 0;
        }
        let Ok(val) = line.parse::<u32>() else { continue };
        acc += val;
    }
    sums.push(acc);
    sums.sort_unstable();
    println!("{}", sums.into_iter().rev().take(3).sum::<u32>());
    Ok(())
}
