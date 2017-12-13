use std::fs::File;
use std::io::Read;
use std::str::Chars;

fn scan_garbage(chars: &mut Chars) -> u64 {
    let mut garbage = 0;
    while let Some(c) = chars.next() {
        match c {
            '>' => break,
            '!' => {
                chars.next();
            }
            _ => garbage += 1,
        }
    }
    return garbage;
}
fn score_groups_iter(mut chars: &mut Chars, depth: u64) -> (u64, u64) {
    let mut score = depth;
    let mut garbage = 0;
    while let Some(c) = chars.next() {
        match c {
            '{' => {
                let (sub_score, sub_garbage) = score_groups_iter(chars, depth + 1);
                score += sub_score;
                garbage += sub_garbage;
            }
            '<' => garbage += scan_garbage(&mut chars),
            '}' => break,
            ',' => (),
            _ => panic!("Unexpected char: {}", c),
        }
    }
    return (score, garbage);
}
fn score_groups(input: &str) -> (u64, u64) {
    let mut chars = input.chars();
    chars.next();
    return score_groups_iter(&mut chars, 1);
}

fn main() {
    let mut file = File::open("day9.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let (score, garbage) = score_groups(&contents);
    println!("Part 1: {}", score);
    println!("Part 2: {}", garbage);
}

#[cfg(test)]
mod tests {
    use score_groups;
    #[test]
    fn test_score() {
        assert_eq!(score_groups("{}").0, 1);
        assert_eq!(score_groups("{{{}}}").0, 6);
        assert_eq!(score_groups("{{},{}}").0, 5);
        assert_eq!(score_groups("{{{},{},{{}}}}").0, 16);
        assert_eq!(score_groups("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(score_groups("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(score_groups("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(score_groups("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn test_garbage() {
        assert_eq!(score_groups("{<>}").1, 0);
        assert_eq!(score_groups("{<random characters>}").1, 17);
        assert_eq!(score_groups("{<<<<>}").1, 3);
        assert_eq!(score_groups("{<{!>}>}").1, 2);
        assert_eq!(score_groups("{<!!>}").1, 0);
        assert_eq!(score_groups("{<!!!>>}").1, 0);
        assert_eq!(score_groups("{<{o\"i!a,<{i<a>}").1, 10);
    }
}
