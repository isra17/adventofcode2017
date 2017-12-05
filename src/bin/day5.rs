use std::fs::File;
use std::io::Read;

fn run_maze(sheet: &str, inc_threshold: Option<i64>) -> u64 {
    let mut jumps: Vec<i64> = sheet.lines().map(|l| l.parse::<i64>().unwrap()).collect();
    let mut i: i64 = 0;
    let mut n = 0;
    while i < jumps.len() as i64 && i >= 0 {
        let j = jumps[i as usize];
        if let Some(t) = inc_threshold {
            if j >= t {
                jumps[i as usize] -= 1;
            } else {
                jumps[i as usize] += 1;
            }
        } else {
            jumps[i as usize] += 1;
        }
        i += j;
        n += 1;
    }
    n
}

fn main() {
    let mut file = File::open("day5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Part 1: {}", run_maze(&contents, None));
    println!("Part 2: {}", run_maze(&contents, Some(3)));
}

#[cfg(test)]
mod tests {
    use run_maze;
    #[test]
    fn test_run_maze1() {
        assert_eq!(run_maze("0\n3\n0\n1\n-3", None), 5);
    }

    #[test]
    fn test_run_maze2() {
        assert_eq!(run_maze("0\n3\n0\n1\n-3", Some(3)), 10);
    }

}
