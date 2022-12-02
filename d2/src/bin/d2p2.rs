use std::io;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Foo(u8);

impl From<String> for Foo {
    fn from(s: String) -> Self {
        Foo(match s.as_str() {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => panic!("unexpected value: {}", s),
        })
    }
}

impl From<Foo> for u32 {
    fn from(f: Foo) -> Self {
        Self::from(f.0)
    }
}

fn work<S: Iterator<Item = String>>(iter: S) -> u32 {
    iter.map(Foo::from).map(u32::from).sum()
}

fn main() -> io::Result<()> {
    let sum = work(io::stdin().lines().filter_map(Result::ok));

    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrapped_work(input: &str) -> u32 {
        work(input.split("\n").map(str::to_owned))
    }

    #[test]
    fn test_work() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(wrapped_work(input), 12);
    }
}
