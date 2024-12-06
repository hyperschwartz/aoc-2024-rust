mod day2 {
    use crate::util::file_loader::load_day_files;

    #[test]
    fn run_day_2() {
        let day = load_day_files(2);
        println!("Part 1 sample: {}", solution_1(&day.sample));
        println!("Part 1 solution: {}", solution_1(&day.problem));
        println!("Part 2 sample: {}", solution_2(&day.sample));
        println!("Part 2 solution: {}", solution_2(&day.problem));
    }

    fn solution_1(lines: &[String]) -> u64 {
        get_input_vecs(lines)
            .into_iter()
            .filter(|list| is_valid_list(list))
            .count() as u64
    }

    fn solution_2(lines: &[String]) -> u64 {
        get_input_vecs(lines)
            .into_iter()
            .filter(|list| is_valid_list(list) || (0..(list.len())).map(|idx| {
                let mut sub_list = list.clone();
                sub_list.remove(idx);
                sub_list
            }).any(|sub_list| is_valid_list(&sub_list)))
            .count() as u64
    }

    fn is_valid_list(list: &[u64]) -> bool {
        (is_ascending(list) || is_descending(list)) && is_spaced_properly(list)
    }

    fn is_ascending(list: &[u64]) -> bool {
        list.windows(2).all(|w| w[0] < w[1])
    }

    fn is_descending(list: &[u64]) -> bool {
        list.windows(2).all(|w| w[0] > w[1])
    }

    fn is_spaced_properly(list: &[u64]) -> bool {
        list.windows(2).map(|w| w[0].abs_diff(w[1])).all(|diff| (1u64..=3u64).contains(&diff))
    }

    fn get_input_vecs(lines: &[String]) -> Vec<Vec<u64>> {
        lines.iter()
            .map(|line| line.split(" ").map(|val| val.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>()
    }
}
