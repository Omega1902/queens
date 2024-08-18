use std::fmt;

use termsize::Size;

struct Queens {
    pub size: u8,
    pub unique_solutions: Vec<u8>,
    pub solutions: Vec<u8>,
}

impl Queens {
    fn default(size: u8) -> Queens {
        Queens { size, unique_solutions: vec![], solutions: vec![] }
    }
}

const QUEEN: char = '♛'; // ♕
const COLORED: char = '█';
const INVISIBLE: char = '░';
const UNKNOWN: char = '?';

#[derive(Clone)]
struct Fields {
    size: u8,
    fixed_rows: Vec<u8>,
    open_rows: Vec<Vec<u8>>,
}

impl fmt::Display for Fields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_rows().join("\n"))
    }
}

impl Fields {
    pub fn get_rows(&self) -> Vec<String> {
        let mut even_row = false;
        let mut result: Vec<String> = self
            .fixed_rows
            .iter()
            .map(|fixed| {
                let mut row: String = String::new();
                for i in 0..self.size {
                    if i == *fixed {
                        row.push(QUEEN);
                    } else if even_row && i % 2 == 0 || !even_row && i % 2 == 1 {
                        row.push(COLORED);
                    } else {
                        row.push(INVISIBLE);
                    }
                }
                even_row = !even_row;
                row
            })
            .collect();
        result.extend(self.open_rows.iter().map(|open| {
            let mut row: String = String::new();
            for i in 0..self.size {
                if open.contains(&i) {
                    row.push(UNKNOWN);
                } else if even_row && i % 2 == 0 || !even_row && i % 2 == 1 {
                    row.push(COLORED);
                } else {
                    row.push(INVISIBLE);
                }
            }
            even_row = !even_row;
            row
        }));
        result
    }

    pub fn default(size: u8) -> Fields {
        Fields { size, fixed_rows: vec![], open_rows: Fields::gen_fields(size) }
    }

    pub fn is_solved(&self) -> bool {
        self.open_rows.is_empty()
    }

    pub fn is_still_possible(&self) -> bool {
        self.open_rows.iter().all(|open_row| !open_row.is_empty())
    }

    pub fn get_next_iteration(&self) -> Vec<Fields> {
        if self.is_solved() {
            return vec![self.clone()];
        }

        // let remaining_base = self.open_rows[1..];
        self.open_rows[0]
            .iter()
            .map(|new_fixed| {
                let mut new_fixed_rows = self.fixed_rows.clone();
                new_fixed_rows.push(*new_fixed);
                let new_open_rows: Vec<Vec<u8>> = self
                    .open_rows
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(number, open_row)| {
                        let n = number as u8;
                        let mut new_open_row = open_row.clone();
                        new_open_row.retain_mut(|x| {
                            if x == new_fixed {
                                return false;
                            }
                            if *x == new_fixed + n {
                                return false;
                            }
                            let temp = new_fixed.checked_sub(n);
                            if temp.is_some() && temp.unwrap() == *x {
                                return false;
                            }
                            true
                        });
                        new_open_row
                    })
                    .collect();
                Fields { size: self.size, fixed_rows: new_fixed_rows, open_rows: new_open_rows }
            })
            .filter(|field| field.is_still_possible())
            .collect()
    }

    pub fn get_all_solutions(&self) -> Vec<Fields> {
        if self.is_solved() {
            return vec![self.clone()];
        }
        let mut result = vec![];
        for next_iteration in self.get_next_iteration() {
            result.extend(next_iteration.get_all_solutions())
        }
        result
    }

    fn gen_fields(num: u8) -> Vec<Vec<u8>> {
        if num == 0 {
            return vec![];
        }
        if num == 1 {
            return vec![vec![0]];
        }
        (0..num).map(|count| if count == 0 { (0..(num + 1) / 2).collect() } else { (0..num).collect() }).collect()
    }
}

fn print_fields(fields: &[Fields]) {
    let width = termsize::get().unwrap_or(Size { rows: 1, cols: 1 }).cols;
    let mut index = 0;
    while index < fields.len() {
        let start_index = index;
        let mut fields_to_print = &fields[start_index..=index];
        let mut current_width = fields_to_print[0].size as u16;
        index += 1;
        while index < fields.len() && current_width + 2 + (fields[index].size as u16) < width {
            fields_to_print = &fields[start_index..=index];
            current_width += 2 + (fields[index].size as u16);
            index += 1;
        }
        let field_strings: Vec<Vec<String>> = fields_to_print.iter().map(|field| field.get_rows()).collect();
        let max_size = fields_to_print.iter().map(|field| field.size).max().unwrap_or(fields_to_print[0].size);
        for line in 0..max_size {
            for (number, field) in field_strings.iter().enumerate() {
                print!("{}", field.get(line as usize).unwrap_or(&(" ".repeat(fields_to_print[number].size as usize))));
                if number + 1 < field_strings.len() {
                    print!("  ");
                }
            }
            println!();
        }
        if index < fields.len() {
            println!();
        }
    }
}

fn calc_queens(num: u8) -> Queens {
    Queens::default(num)
}

fn calc_and_print(size: u8, print_solutions: bool) {
    println!("Calculating solutions for {} Queens:", size);
    let all_solutions = Fields::default(size).get_all_solutions();
    if print_solutions {
        print_fields(&all_solutions);
    }
    println!("Solved {} Queens solutions ({})", size, all_solutions.len());
}

fn main() {
    println!("Unsolved 1 Queen: {}", Fields { size: 1, fixed_rows: vec![], open_rows: vec![vec![0]] });
    println!("Solved 1 Queen: {}", Fields { size: 1, fixed_rows: vec![0], open_rows: vec![] });
    println!("Solved 4 Queens:\n{}", Fields { size: 4, fixed_rows: vec![1, 3, 0, 2], open_rows: vec![] });
    println!("Unsolved 1 Queen {}", Fields::default(1));
    println!("Unsolved 4 Queens \n{}", Fields::default(4));

    let mut next_iteration = Fields::default(4).get_next_iteration();
    println!("Unsolved 4 Queens next iteration ({}):", next_iteration.len());
    print_fields(&next_iteration);

    println!("Test some stuff:");
    next_iteration.insert(1, Fields::default(1));
    next_iteration.extend(Fields::default(5).get_next_iteration());
    print_fields(&next_iteration);

    next_iteration = Fields::default(8).get_next_iteration();
    println!("Unsolved 8 Queens next iteration ({}):", next_iteration.len());
    print_fields(&next_iteration);

    calc_and_print(4, true);
    println!();
    calc_and_print(8, false);
    println!();
    calc_and_print(10, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved() {
        assert!(Fields::default(0).is_solved());
        assert!(Fields { size: 1, fixed_rows: vec![0], open_rows: vec![] }.is_solved());
        assert!(!Fields { size: 1, fixed_rows: vec![], open_rows: vec![vec![0]] }.is_solved());
        assert!(Fields { size: 4, fixed_rows: vec![1, 3, 0, 2], open_rows: vec![] }.is_solved());
        assert!(!Fields { size: 4, fixed_rows: vec![1, 3, 0], open_rows: vec![vec![2]] }.is_solved());
    }

    #[test]
    fn test_is_possible() {
        assert!(Fields { size: 1, fixed_rows: vec![0], open_rows: vec![] }.is_still_possible());
        assert!(Fields { size: 1, fixed_rows: vec![], open_rows: vec![vec![0]] }.is_still_possible());
        assert!(Fields { size: 4, fixed_rows: vec![1, 3, 0, 2], open_rows: vec![] }.is_still_possible());
        assert!(Fields { size: 4, fixed_rows: vec![1, 3, 0], open_rows: vec![vec![2]] }.is_still_possible());
        assert!(!Fields { size: 4, fixed_rows: vec![0, 2], open_rows: vec![vec![], vec![1, 3]] }.is_still_possible());
    }

    #[test]
    fn test_gen_fields() {
        assert_eq!(Fields::gen_fields(1), vec![vec![0]]);
        assert_eq!(Fields::gen_fields(2), vec![vec![0], vec![0, 1]]);
        assert_eq!(Fields::gen_fields(4), vec![vec![0, 1], vec![0, 1, 2, 3], vec![0, 1, 2, 3], vec![0, 1, 2, 3]]);
        assert_eq!(
            Fields::gen_fields(5),
            vec![vec![0, 1, 2], vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4]]
        );
        assert_eq!(
            Fields::gen_fields(8),
            vec![
                vec![0, 1, 2, 3],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![0, 1, 2, 3, 4, 5, 6, 7],
                vec![0, 1, 2, 3, 4, 5, 6, 7]
            ]
        );
    }
}
