mod day6 {
    use crate::util::file_loader::load_day_files;

    #[test]
    fn run_day_6() {
        let day = load_day_files(6);
        println!("Part 1 sample: {}", solution_1(&day.sample));
        println!("Part 1 solution: {}", solution_1(&day.problem));
        println!("Part 2 sample: {}", solution_2(&day.sample));
        println!("Part 2 solution: {}", solution_2(&day.problem));
    }

    fn solution_1(lines: &[String]) -> usize {
        let (grid, guard) = parse_input(lines);
        get_visited_grid(grid, guard).iter().map(|row| row.iter().filter(|tile| tile.visited).count()).sum()
    }

    fn get_visited_grid(mut grid: Vec<Vec<Tile>>, mut guard: Guard) -> Vec<Vec<Tile>> {
        let max_x = (grid.first().unwrap().len() - 1) as i32;
        let max_y = (grid.len() - 1) as i32;
        loop {
            let (next_x, next_y) = guard.next_location();
            // If the guard has left the building, peace out
            if next_x < 0 || next_y < 0 || next_x > max_x || next_y > max_y {
                break;
            }
            let mut tile = get_item_at(&mut grid, next_x as usize, next_y as usize);
            match tile.tile_type {
                TileType::Ground => {
                    tile.visit(&guard.current_direction);
                    guard.move_to_next_location();
                },
                TileType::Obstacle => {
                    guard.rotate();
                }
            }
        }
        grid
    }

    fn solution_2(lines: &[String]) -> u64 {
        let (grid, guard) = parse_input(lines);
        let max_x = (grid.first().unwrap().len() - 1) as i32;
        let max_y = (grid.len() - 1) as i32;
        let visited_grid = get_visited_grid(grid.clone(), guard.clone());
        let mut possible_obstacles: Vec<Point> = vec![];
        visited_grid.iter().enumerate().for_each(|(y, tile_vec)| {
            tile_vec.iter().enumerate().for_each(|(x, tile)| {
                if tile.visited {
                    let point = Point::new(x, y);
                    if !possible_obstacles.iter().any(|ob| ob.x == point.x && ob.y == point.y) {
                        if point.x != guard.pos_x || point.y != guard.pos_y {
                            possible_obstacles.push(point);
                        }
                    }
                }
            });
        });
        let mut solution_count: u64 = 0;
        let mut attempt_count: u64 = 0;
        for new_obstacle in possible_obstacles.iter() {
            attempt_count += 1;
            let mut current_grid = grid.clone();
            let altered_tile = get_item_at(&mut current_grid, new_obstacle.x, new_obstacle.y);
            match altered_tile.tile_type {
                // If the potential new obstacle already is an obstacle, it cannot possibly be a solution
                TileType::Obstacle => continue,
                TileType::Ground => {
                    altered_tile.tile_type = TileType::Obstacle;
                }
            };
            let mut current_guard = guard.clone();
            loop {
                let (next_x, next_y) = current_guard.next_location();
                if next_x < 0 || next_y < 0 || next_x > max_x || next_y > max_y {
                    break;
                }
                let mut tile = get_item_at(&mut current_grid, next_x as usize, next_y as usize);
                tile.visit(&current_guard.current_direction);
                if tile.has_duplicate_visits() {
                    solution_count += 1;
                    break;
                }
                match tile.tile_type {
                    TileType::Ground => {
                        current_guard.move_to_next_location();
                    } TileType::Obstacle => {
                        current_guard.rotate();
                    }
                }
            }
        }
        solution_count
    }

    fn parse_input(lines: &[String]) -> (Vec<Vec<Tile>>, Guard) {
        let mut guard = Guard { pos_x: 0, pos_y: 0, current_direction: MovementDirection::North };
        let mut grid: Vec<Vec<Tile>> = vec![];
        lines.iter().enumerate().for_each(|(y, line)| {
            let mut current_row: Vec<Tile> = vec![];
            line.chars().enumerate().for_each(|(x, c)| {
                let (visited, tile_type) = match c {
                    '#' => (false, TileType::Obstacle),
                    '.' => (false, TileType::Ground),
                    '^' => {
                        guard.pos_x = x;
                        guard.pos_y = y;
                        (true, TileType::Ground)
                    },
                    _ => panic!("{}", format!("unexpected character {c} in scanned grid")),
                };
                current_row.push(Tile::new(tile_type, x, y, visited));
            });
            grid.push(current_row);
        });
        (grid, guard)
    }

    fn get_item_at(grid: &mut [Vec<Tile>], x: usize, y: usize) -> &mut Tile {
        grid.get_mut(y).and_then(|vec| vec.get_mut(x)).expect(&format!("expected a tile to exist at coordinate (x={x}, y={y})"))
    }

    #[derive(Clone)]
    enum TileType {
        Ground,
        Obstacle,
    }

    #[derive(Clone)]
    struct Tile {
        tile_type: TileType,
        visited: bool,
        x: usize,
        y: usize,
        north_visits: i32,
        south_visits: i32,
        east_visits: i32,
        west_visits: i32,
    }

    impl Tile {
        fn new(tile_type: TileType, x: usize, y: usize, visited: bool) -> Tile {
            Tile {
                tile_type,
                visited,
                x,
                y,
                north_visits: if visited { 1 } else { 0 },
                south_visits: 0,
                east_visits: 0,
                west_visits: 0,
            }
        }

        fn visit(&mut self, direction: &MovementDirection) {
            self.visited = true;
            match direction {
                MovementDirection::North => self.north_visits += 1,
                MovementDirection::South => self.south_visits += 1,
                MovementDirection::East => self.east_visits += 1,
                MovementDirection::West => self.west_visits += 1,
            }
        }

        fn has_duplicate_visits(&self) -> bool {
            self.north_visits > 1 ||
                self.south_visits > 1 ||
                self.east_visits > 1 ||
                self.west_visits > 1
        }
    }

    #[derive(Clone)]
    enum MovementDirection {
        North,
        South,
        East,
        West,
    }

    impl MovementDirection {
        fn value(&self) -> (i32, i32) {
            match self {
                MovementDirection::North => (0, -1),
                MovementDirection::South => (0 , 1),
                MovementDirection::East => (1, 0),
                MovementDirection::West => (-1, 0),
            }
        }
    }

    #[derive(Clone)]
    struct Guard {
        pos_x: usize,
        pos_y: usize,
        current_direction: MovementDirection,
    }

    impl Guard {
        fn next_location(&self) -> (i32, i32) {
            let (x_inc, y_inc) = self.current_direction.value();
            (self.pos_x as i32 + x_inc, self.pos_y as i32 + y_inc)
        }

        fn move_to_next_location(&mut self) {
            let (next_x, next_y) = self.next_location();
            self.pos_x = next_x as usize;
            self.pos_y = next_y as usize;
        }

        fn rotate(&mut self) {
            self.current_direction = match self.current_direction {
                MovementDirection::North => MovementDirection::East,
                MovementDirection::East => MovementDirection::South,
                MovementDirection::South => MovementDirection::West,
                MovementDirection::West => MovementDirection::North,
            };
        }
    }

    #[derive(PartialEq, Debug)]
    struct Point {
        x: usize,
        y: usize,
    }

    impl Point {
        fn new(x: usize, y: usize) -> Point {
            Point {
                x,
                y,
            }
        }
    }
}
