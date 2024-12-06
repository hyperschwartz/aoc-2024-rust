mod day3 {
    use regex::Regex;
    use crate::util::file_loader::load_day_files;

    #[test]
    fn run_day_3() {
        let day = load_day_files(3);
        println!("Part 1 sample: {}", solution_1(&day.sample));
        println!("Part 1 solution: {}", solution_1(&day.problem));
        println!("Part 2 sample: {}", solution_2(&day.sample));
        println!("Part 2 solution: {}", solution_2(&day.problem));
    }

    fn solution_1(lines: &[String]) -> u64 {
        let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        regex.find_iter(&get_input_text(lines)).map(|m| m.as_str()).map(|mul_problem| apply_mul(mul_problem)).sum()
    }

    fn solution_2(lines: &[String]) -> u64 {
        let regex = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))").unwrap();
        let mut enabled = true;
        regex.find_iter(&get_input_text(lines)).map(|m| m.as_str()).fold(0u64, |acc, line| {
           match line {
               "do()" => {
                   enabled = true;
                   acc
               },
               "don't()" => {
                   enabled = false;
                   acc
               },
               _ => {
                   if enabled {
                       acc + apply_mul(line)
                   } else {
                       acc
                   }
               },
           }
        })
    }

    fn apply_mul(mul_problem: &str) -> u64 {
        mul_problem
            .replace("mul(", "")
            .replace(")", "")
            .split(",")
            .map(|number| number.parse::<u64>().unwrap())
            .fold(0u64, |acc, val| {
                if acc == 0 {
                    val
                } else {
                    acc * val
                }
            })
    }

    fn get_input_text(lines: &[String]) -> String {
        lines.join("")
    }
}
