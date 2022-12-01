use std::io;

fn main() -> io::Result<()> {
    let acc = io::stdin().lines().fold([0, 0], |mut acc, line| {
        let Ok(line) = line else { return acc };
        if line.len() == 0 {
            acc[0] = std::cmp::max(acc[0], acc[1]);
            acc[1] = 0;
            return acc;
        }
        let Ok(v) = line.parse::<u32>() else { return acc };
        acc[1] += v;
        acc
    });
    let result = std::cmp::max(acc[0], acc[1]);
    println!("{}", result);
    Ok(())
}
