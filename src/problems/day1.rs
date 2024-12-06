mod day1 {
    use crate::util::file_loader::load_day_files;

    #[test]
    fn run_day_1() {
        let day = load_day_files(1);
        println!("Part 1 sample: {}", solution_1(&day.sample));
        println!("Part 1 solution: {}", solution_1(&day.problem));
        println!("Part 2 sample: {}", solution_2(&day.sample));
        println!("Part 2 solution: {}", solution_2(&day.problem));
    }

    fn solution_1(lines: &[String]) -> u64 {
        let (mut first_list, mut second_list) = fetch_input_vecs(lines);
        first_list.sort();
        second_list.sort();
        first_list
            .iter()
            .zip(second_list.iter())
            .map(|(first, second)| first.abs_diff(*second))
            .sum::<u64>()
    }

    fn solution_2(lines: &[String]) -> u64 {
        let (first_list, second_list) = fetch_input_vecs(lines);
        first_list
            .iter()
            .map(|first| first * (second_list.iter().filter(|second| *second == first).count() as u64))
            .sum()
    }

    fn fetch_input_vecs(lines: &[String]) -> (Vec<u64>, Vec<u64>) {
        let pair_map = lines.iter().map(|line| {
            let value = line.trim().split(" ").filter(|value| value.trim() != "").collect::<Vec<&str>>();
            (value[0].parse::<u64>().unwrap(), value[1].parse::<u64>().unwrap())
        });
        (pair_map.clone().map(|pair| pair.0).collect::<Vec<u64>>(), pair_map.map(|pair| pair.1).collect::<Vec<u64>>())
    }
}
