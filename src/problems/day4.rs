mod day4 {
    use std::fmt::Debug;
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    use crate::util::file_loader::load_day_files;

    #[test]
    fn run_day_4() {
        let day = load_day_files(4);
        println!("Part 1 sample: {}", solution_1(&day.sample));
        println!("Part 1 solution: {}", solution_1(&day.problem));
        println!("Part 2 sample: {}", solution_2(&day.sample));
        println!("Part 2 solution: {}", solution_2(&day.problem));
    }

    fn solution_1(lines: &[String]) -> u64 {
        let mut xmas_count = 0u64;
        let grid = get_grid(lines);
        for (y, char_line) in grid.iter().enumerate() {
            for (x, char) in char_line.clone().into_iter().enumerate() {
                if char == 'X' {
                    XMasDirection::iter().for_each(|direction| {
                        xmas_count += xmas_found(x as i64, y as i64, &grid, 'X', direction);
                    });
                }
            }
        }
        xmas_count
    }

    fn solution_2(lines: &[String]) -> u64 {
        let mut xmas_count = 0u64;
        let grid = get_grid(lines);
        for (y, char_line) in grid.iter().enumerate() {
            for (x, char) in char_line.clone().into_iter().enumerate() {
                if char == 'A' {
                    if let Some(north_west) = char_at_direct(&grid, x, y, XMasDirection::NorthWest) {
                        if let Some(north_east) = char_at_direct(&grid, x, y, XMasDirection::NorthEast) {
                            if let Some(south_west) = char_at_direct(&grid, x, y, XMasDirection::SouthWest) {
                                if let Some(south_east) = char_at_direct(&grid, x, y, XMasDirection::SouthEast) {
                                    let count_vec = vec![north_west, south_east, north_east, south_west];
                                    if count_vec.iter().filter(|c| **c == &'S').count() != 2 {
                                        continue;
                                    }
                                    if count_vec.iter().filter(|c| **c == &'M').count() != 2 {
                                        continue;
                                    }
                                    if north_west != south_east && north_east != south_west {
                                        xmas_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        xmas_count
    }

    fn xmas_found(x: i64, y: i64, grid: &[Vec<char>], current_char: char, direction: XMasDirection) -> u64 {
        let target_char = get_next_char(current_char).unwrap();
        if target_char == 'Z' {
            return 1;
        }
        let (x_inc, y_inc) = direction.value();
        let target_x = x + x_inc;
        let target_y = y + y_inc;
        if target_x < 0 || target_y < 0 {
            return 0;
        }
        char_at(grid, target_x as usize, target_y as usize).map(|selected_char| {
            if selected_char == &target_char {
                xmas_found(target_x, target_y, grid, target_char, direction)
            } else {
                0
            }
        }).unwrap_or(0)
    }

    fn char_at(grid: &[Vec<char>], x: usize, y: usize) -> Option<&char> {
        grid.get(y).and_then(|char_list| char_list.get(x))
    }

    fn char_at_direct(grid: &[Vec<char>], x: usize, y: usize, direction: XMasDirection) -> Option<&char> {
        let (x_inc, y_inc) = direction.value();
        let target_x = x as i64 + x_inc;
        let target_y = y as i64 + y_inc;
        if target_x < 0 || target_y < 0 {
            return None;
        }
        char_at(grid, target_x as usize, target_y as usize)
    }

    #[derive(Debug, EnumIter)]
    enum XMasDirection {
        North,
        NorthWest,
        NorthEast,
        East,
        West,
        South,
        SouthWest,
        SouthEast,
    }

    impl XMasDirection {
        fn value(&self) -> (i64, i64) {
            match *self {
                XMasDirection::North => (0, -1),
                XMasDirection::NorthWest => (-1, -1),
                XMasDirection::NorthEast => (1, -1),
                XMasDirection::East => (1, 0),
                XMasDirection::West => (-1, 0),
                XMasDirection::South => (0, 1),
                XMasDirection::SouthWest => (-1, 1),
                XMasDirection::SouthEast => (1, 1),
            }
        }
    }

    fn get_next_char(current_char: char) -> Option<char> {
        match current_char {
            'X' => Some('M'),
            'M' => Some('A'),
            'A' => Some('S'),
            'S' => Some('Z'),
            _ => None,
        }
    }

    fn get_grid(lines: &[String]) -> Vec<Vec<char>> {
        lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect()
    }
}
