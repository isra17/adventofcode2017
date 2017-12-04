use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn validate_passphrases(passphrases: &str, check_anagram: bool) -> u64 {
    passphrases.lines()
        .map(|l| l.split_whitespace())
        .map(|ws| {
            ws.scan(HashSet::new(), |s, w| {
                    let mut ww = String::from(w);
                    if check_anagram {
                        unsafe {
                            let v = ww.as_mut_vec();
                            v.sort();
                        }
                    }

                    if s.contains(&ww) {
                        Some(false)
                    } else {
                        s.insert(ww);
                        Some(true)
                    }
                })
                .all(|r| r)
        })
        .map(|uniq| if uniq {
            1
        } else {
            0
        })
        .sum()
}

fn main() {
    let mut file = File::open("day4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Part1: {}", validate_passphrases(&contents, false));
    println!("Part2: {}", validate_passphrases(&contents, true));
}

#[cfg(test)]
mod tests {
    use validate_passphrases;
    #[test]
    fn test_validate_passphrases() {
        assert_eq!(validate_passphrases("aa bb cc dd ee".into(), false), 1);
        assert_eq!(validate_passphrases("aa bb cc dd aa".into(), false), 0);
        assert_eq!(validate_passphrases("aa bb cc dd aaa".into(), false), 1);
    }
    #[test]
    fn test_validate_passphrases_with_anagrams() {
        assert_eq!(validate_passphrases("abcde fghij".into(), true), 1);
        assert_eq!(validate_passphrases("abcde xyz ecdab".into(), true), 0);
        assert_eq!(validate_passphrases("a ab abc abd abf abj".into(), true), 1);
        assert_eq!(validate_passphrases("iiii oiii ooii oooi oooo".into(), true),
                   1);
        assert_eq!(validate_passphrases("oiii ioii iioi iiio".into(), true), 0);
    }
}
