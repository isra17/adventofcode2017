use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn process(program: &str) -> (i64, i64) {
    let mut lines = program.lines().map(|l| l.split_whitespace());
    let mut registers: HashMap<&str, i64> = HashMap::new();
    let mut max = 0;
    while let Some(mut words) = lines.next() {
        let register = words.next().unwrap();
        let op = words.next().unwrap();
        let value = words.next().unwrap().parse::<i64>().unwrap();
        words.next();
        let cond_register = words.next().unwrap();
        let cond_op = words.next().unwrap();
        let cond_value = words.next().unwrap().parse::<i64>().unwrap();

        let cond_reg_value = *registers.get(cond_register).unwrap_or(&0);
        let cond = match cond_op {
            ">" => cond_reg_value > cond_value,
            "<" => cond_reg_value < cond_value,
            "==" => cond_reg_value == cond_value,
            "!=" => cond_reg_value != cond_value,
            "<=" => cond_reg_value <= cond_value,
            ">=" => cond_reg_value >= cond_value,
            _ => panic!("Unexpected {}", cond_op),
        };

        if cond {
            let reg_value = *registers.get(register).unwrap_or(&0);
            registers.insert(register,
                             match op {
                                 "inc" => reg_value + value,
                                 "dec" => reg_value - value,
                                 _ => panic!("Unexpected {}", cond_op),
                             });

            if reg_value > max {
                max = reg_value;
            }
        }
    }

    (*registers.values().max().unwrap(), max)
}

fn main() {
    let mut file = File::open("day8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let (final_max, all_time_max) = process(&contents);
    println!("Part 1: {}", final_max);
    println!("Part 2: {}", all_time_max);
}

#[cfg(test)]
mod tests {
    use process;
    #[test]
    fn test_process() {
        assert_eq!(process("b inc 5 if a > 1
                            a inc 1 if b < 5
                            c dec -10 if a >= 1
                            c inc -20 if c == 10"),
                   (1, 10));
    }


}
