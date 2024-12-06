use std::fs::read_to_string;

pub struct DayFiles {
    pub day_number: u8,
    pub sample: Vec<String>,
    pub problem: Vec<String>,
}

pub fn load_day_files(day_number: u8) -> DayFiles {
    DayFiles {
        day_number,
        sample: load_sample_file(day_number),
        problem: load_problem_file(day_number),
    }
}

pub fn load_sample_file(day_number: u8) -> Vec<String> {
    load_file(day_number, "sample.txt")
}

pub fn load_problem_file(day_number: u8) -> Vec<String> {
    load_file(day_number, "problem.txt")
}

fn load_file(day_number: u8, file_name: &str) -> Vec<String> {
    load_file_by_name(&format!("day{day_number}"), file_name)
}

fn load_file_by_name(day_name: &str, file_name: &str) -> Vec<String> {
    let file_path = format!("inputs/{day_name}/{file_name}");
    read_to_string(&file_path)
        .expect(&format!("expected a file to exist at path {file_path}"))
        .lines()
        .map(String::from)
        .collect()
}

mod tests {
    use crate::util::file_loader::{load_day_files, load_file_by_name};

    #[test]
    fn test_load_files() {
        let problem = load_file_by_name("test", "problem.txt");
        assert_eq!(vec!["problem".to_string()], problem, "expected the proper problem values to be parsed");
        let sample = load_file_by_name("test", "sample.txt");
        assert_eq!(vec!["sample".to_string(), "sample2".to_string()], sample, "expected the proper sample values to be parsed");
    }

    #[test]
    fn test_load_day_files() {
        let day1 = load_day_files(1);
        assert_eq!(1, day1.day_number, "the proper day number should be returned");
        assert!(!day1.sample.is_empty(), "sample lines for day 1 should be properly scanned");
        assert!(!day1.problem.is_empty(), "problem lines for day 1 should be properly scanned");
    }
}
