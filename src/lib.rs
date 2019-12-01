use std::io;

fn calc(mass: u32) -> u32 {
    (mass / 3) - 2
}

pub fn sum(r: impl io::BufRead) -> io::Result<u32> {
    let mut fuel = 0;
    for line in r.lines() {
        let mass = line.unwrap().parse::<u32>().unwrap();
        fuel += calc(mass);
    }
    Ok(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_examples() {
        assert_eq!(calc(12), 2);
        assert_eq!(calc(14), 2);
        assert_eq!(calc(1969), 654);
        assert_eq!(calc(100756), 33583);
    }

    #[test]
    fn sum_examples() {
        let b = "12
14
1969
100756"
            .as_bytes();
        assert_eq!(sum(b).unwrap(), 2 + 2 + 654 + 33583);
    }
}
