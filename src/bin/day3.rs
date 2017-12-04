fn solve1(n: i64) -> i64 {
    let mut base = (n as f64).sqrt().ceil() as i64;

    if base % 2 == 0 {
        base = base + 1;
    }

    if base == 1 {
        return 0;
    }

    let prev_square = (base - 2) * (base - 2);
    let side = (n - prev_square) / (base - 1);
    let center = prev_square + base / 2 + (side * (base - 1));
    (n - center).abs() + (base - 1) / 2
}

// struct Spiral {
// data: Vec<i64>,
// data_base: i64,
// cursor_x: i64,
// cursor_y: i64,
// }
//
// impl Spiral {
// fn new(size: usize) -> Spiral {
// let spiral = Spiral {
// data: Vec::with_capacity(size * size),
// data_base: size,
// cursor_x: 0,
// cursor_y: 0,
// };
// spiral.data.resize(size * size, 0);
// spiral
// }
//
// fn push(self, value: i64) {}
// }
//

struct SpiralSum {
    cache: Vec<i64>,
}

impl SpiralSum {
    fn new() -> SpiralSum {
        SpiralSum { cache: Vec::new() }
    }

    fn get_value(&self, n: i64) -> i64 {
        if n == 1 {
            return 1;
        } else if n <= 0 {
            return 0;
        }
        let mut base = (n as f64).sqrt().ceil() as i64;
        if base % 2 == 0 {
            base = base + 1;
        }
        let prev_square = (base - 2) * (base - 2);
        let side = (n - prev_square) / (base - 1);
        0
    }
}

fn solve2(n: i64) -> i64 {
    let mut i = 1;
    let spiral = SpiralSum::new();
    loop {
        if spiral.get_value(i) > n {
            return spiral.get_value(i);
        }
        i += 1;
    }

}

fn main() {
    println!("Part 1: {}", solve1(325489));
    println!("Part 2: {}", solve2(325489));
}

#[cfg(test)]
mod tests {
    use solve1;
    use solve2;
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(1), 0);
        assert_eq!(solve1(12), 3);
        assert_eq!(solve1(23), 2);
        assert_eq!(solve1(1024), 31);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(0), 1);
        assert_eq!(solve2(1), 2);
        assert_eq!(solve2(2), 4);
        assert_eq!(solve2(4), 5);
        assert_eq!(solve2(5), 10);
        assert_eq!(solve2(10), 11);
        assert_eq!(solve2(11), 23);
        assert_eq!(solve2(362), 747);
    }
}
