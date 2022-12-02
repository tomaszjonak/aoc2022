use std::io;

fn main() -> io::Result<()> {
    let mut sums = io::stdin().lines().fold(vec![0], |mut acc, line| {
        let Ok(line) = line else { return acc };
        if line.len() == 0 {
            acc.push(0);
            return acc;
        }
        let Ok(v) = line.parse::<u32>() else { return acc };
        acc.last_mut().map(|x| *x += v);
        acc
    });
    sums.sort_unstable();
    println!("{}", sums.into_iter().rev().take(3).sum::<u32>());
    Ok(())
}
