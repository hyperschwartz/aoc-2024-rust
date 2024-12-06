mod day5 {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::collections::HashMap;
    use crate::util::file_loader::load_day_files;

    #[test]
    fn run_day_5() {
        let day = load_day_files(5);
        println!("Part 1 sample: {}", solution_1(&day.sample));
        println!("Part 1 solution: {}", solution_1(&day.problem));
        println!("Part 2 sample: {}", solution_2(&day.sample));
        println!("Part 2 solution: {}", solution_2(&day.problem));
    }

    fn solution_1(lines: &[String]) -> u16 {
        let (orders, inputs) = parse_input(lines);
        let (correct_lists, _) = get_sorted_and_unsorted(&orders, &inputs);
        correct_lists.iter().map(|list| list[(list.len() - 1) / 2]).sum()
    }

    fn solution_2(lines: &[String]) -> u16 {
        let (orders, inputs) = parse_input(lines);
        let (_, incorrect_lists) = get_sorted_and_unsorted(&orders, &inputs);
        let mut solution_count = 0;
        incorrect_lists.iter().for_each(|incorrect| {
            let mut sorted_list = incorrect.clone();
            sorted_list.sort_by(|first, second| {
                if let Some(order_list) = orders.get(first) {
                    if order_list.contains(second) {
                        Less
                    } else {
                        Greater
                    }
                } else {
                    Equal
                }
            });
            solution_count += sorted_list[(sorted_list.len() - 1) / 2]
        });
        solution_count
    }

    fn get_sorted_and_unsorted(orders: &HashMap<u16, Vec<u16>>, inputs: &Vec<Vec<u16>>) -> (Vec<Vec<u16>>, Vec<Vec<u16>>) {
        let mut correct_lists: Vec<Vec<u16>> = vec![];
        let mut incorrect_lists: Vec<Vec<u16>> = vec![];
        inputs.iter().for_each(|input_list| {
            let mut list_is_sorted = true;
            input_list.iter().enumerate().for_each(|(idx, value)| {
                if list_is_sorted {
                    if let Some(required_values) = orders.get(value) {
                        if input_list.iter().take(idx).any(|value| required_values.contains(value)) {
                            list_is_sorted = false;
                        }
                    }
                }
            });
            if list_is_sorted {
                correct_lists.push(input_list.clone());
            } else {
                incorrect_lists.push(input_list.clone());
            }
        });
        (correct_lists, incorrect_lists)
    }

    fn parse_input(lines: &[String]) -> (HashMap<u16, Vec<u16>>, Vec<Vec<u16>>) {
        let (split_index, _) = lines.iter().enumerate().find(|line| line.1.is_empty()).unwrap();
        let first_input = lines.iter().take(split_index).map(|s| s.as_str()).collect::<Vec<&str>>();
        let second_input = &lines[split_index+1..lines.len()];
        let mut order_map: HashMap<u16, Vec<u16>> = HashMap::new();
        first_input
            .iter()
            .map(|line| {
                line.split("|").map(|value| value.parse::<u16>().unwrap()).collect::<Vec<u16>>()
            })
            .map(|line_values| (line_values[0], line_values[1]))
            .for_each(|(x, y)| {
                match order_map.get_mut(&x) {
                    Some(vec) => vec.push(y),
                    None => {
                        order_map.insert(x, vec![y]);
                    },
                };
            });
        let test_vec = second_input
            .iter()
            .map(|line| {
                line.split(",").map(|value| value.parse::<u16>().unwrap()).collect::<Vec<u16>>()
            })
            .collect();
        (order_map, test_vec)
    }
}
