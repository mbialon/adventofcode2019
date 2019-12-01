use std::io;

fn calc(mass: i32, total: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 {
        calc(fuel, total + fuel)
    } else {
        total
    }
}

pub fn sum(r: impl io::BufRead) -> io::Result<i32> {
    let mut fuel = 0;
    for line in r.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        fuel += calc(mass, 0);
    }
    Ok(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_examples() {
        assert_eq!(calc(12, 0), 2);
        assert_eq!(calc(14, 0), 2);
        assert_eq!(calc(1969, 0), 966);
        assert_eq!(calc(100756, 0), 50346);
    }

    #[test]
    fn sum_examples() {
        let b = "12
14
1969
100756"
            .as_bytes();
        assert_eq!(sum(b).unwrap(), 2 + 2 + 966 + 50346);
    }
}
