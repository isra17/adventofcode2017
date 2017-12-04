
fn coord_for(n: i64) -> (i64, i64) {
    let mut base = (n as f64).sqrt().ceil() as i64;

    if base % 2 == 0 {
        base = base + 1;
    }

    if base == 1 {
        return (0, 0);
    }

    let prev_square = (base - 2) * (base - 2);
    let side = (n - prev_square) / (base - 1);
    let center = prev_square + base / 2 + (side * (base - 1));
    let a = n - center;
    let b = (base - 1) / 2;

    match side {
        0 | 4 => (b, a),
        1 => (-a, b),
        2 => (-b, -a),
        3 => (a, -b),
        _ => unreachable!(),
    }

}
fn solve1(n: i64) -> i64 {
    let (x, y) = coord_for(n);
    x.abs() + y.abs()
}

struct Spiral {
    data: Vec<i64>,
    size: usize,
    cursor: usize,
}

static DIRECTIONS: &'static [&'static [i64]] = &[&[-1, -1], &[0, -1], &[1, -1], &[-1, 0], &[0, 0],
                                                 &[1, 0], &[-1, 1], &[0, 1], &[1, 1]];

impl Spiral {
    fn new(size: usize) -> Spiral {
        let mut spiral = Spiral {
            data: Vec::with_capacity(size * size),
            size: size,
            cursor: 1,
        };
        spiral.data.resize(size * size, 0);
        let center = size / 2;
        spiral.data[center + center * size] = 1;
        spiral
    }

    fn next(&mut self) -> Option<i64> {
        if self.cursor >= self.size * self.size {
            return None;
        }

        let (x, y) = coord_for(self.cursor as i64);
        self.cursor += 1;

        let center = self.size as i64 / 2;
        let value = DIRECTIONS.iter()
            .map(|d| {
                self.data[(d[0] + center + x) as usize + (d[1] + center + y) as usize * self.size]
            })
            .sum();
        self.data[(center + x) as usize + (center + y) as usize * self.size] = value;
        Some(value)
    }
}

fn solve2(n: i64) -> i64 {
    let mut spiral = Spiral::new(11);
    loop {
        let v = spiral.next().unwrap() as i64;
        if v > n {
            return v;
        }
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
    use coord_for;
    use Spiral;

    #[test]
    fn test_coord() {
        assert_eq!(coord_for(9), (1, -1));
        assert_eq!(coord_for(12), (2, 1));
        assert_eq!(coord_for(14), (1, 2));
        assert_eq!(coord_for(20), (-2, -1));
        assert_eq!(coord_for(21), (-2, -2));
        assert_eq!(coord_for(22), (-1, -2));
        assert_eq!(coord_for(23), (0, -2));
        assert_eq!(coord_for(25), (2, -2));
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(1), 0);
        assert_eq!(solve1(12), 3);
        assert_eq!(solve1(23), 2);
        assert_eq!(solve1(1024), 31);
    }

    #[test]
    fn test_spiral() {
        let mut spiral = Spiral::new(11);
        assert_eq!(spiral.next(), Some(1));
        assert_eq!(spiral.next(), Some(1));
        assert_eq!(spiral.next(), Some(2));
        assert_eq!(spiral.next(), Some(4));
        assert_eq!(spiral.next(), Some(5));
        assert_eq!(spiral.next(), Some(10));
        assert_eq!(spiral.next(), Some(11));
        assert_eq!(spiral.next(), Some(23));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(5), 10);
        assert_eq!(solve2(11), 23);
        assert_eq!(solve2(23), 25);
        assert_eq!(solve2(25), 26);
        assert_eq!(solve2(59), 122);
        assert_eq!(solve2(147), 304);
        assert_eq!(solve2(747), 806);
    }
}
