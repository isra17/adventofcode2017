use std::collections::HashMap;

fn distribute_banks(input: &str) -> (u64, u64) {
    let mut seen = HashMap::<Vec<u64>, u64>::new();
    let mut n = 0;
    let mut state: Vec<u64> = input.split_whitespace().map(|w| w.parse::<u64>().unwrap()).collect();
    while !seen.contains_key(&state) {
        seen.insert(state.clone(), n);
        let (max, max_i) = state.iter()
            .enumerate()
            .fold((0, 0), |(mx, mi), (i, &x)| if x > mx {
                (x, i)
            } else {
                (mx, mi)
            });

        // println!("max for {:?}: {} at {}", state, max, max_i);
        state[max_i] = 0;
        let size = state.len();
        for i in 0..state.len() {
            state[(i + max_i + 1) % size] += max / size as u64 + (i < max as usize % size) as u64;
        }
        n += 1;
    }

    (n, n - seen[&state])
}

fn main() {
    let input = "4   10  4   1   8   4   9   14  5   1   14  15  0   15  3   5";

    let (cycle, second_cycle) = distribute_banks(&input);
    println!("Part 1: {}", cycle);
    println!("Part 2: {}", second_cycle);
}

#[cfg(test)]
mod tests {
    use distribute_banks;
    #[test]
    fn test_distribute_banks() {
        assert_eq!(distribute_banks("0 2 7 0"), (5, 4));
    }

}
