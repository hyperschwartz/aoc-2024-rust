mod day7 {
    use crate::util::file_loader::load_day_files;

    #[test]
    fn run_day_7() {
        let day = load_day_files(7);
        println!("Part 1 sample: {}", solution_1(&day.sample));
        println!("Part 1 solution: {}", solution_1(&day.problem));
        println!("Part 2 sample: {}", solution_2(&day.sample));
        println!("Part 2 solution: {}", solution_2(&day.problem));
    }

    fn solution_1(lines: &[String]) -> usize {
        0
    }

    fn solution_2(lines: &[String]) -> usize {
        0
    }

    fn parse_lines(lines: &[String]) {}
}
