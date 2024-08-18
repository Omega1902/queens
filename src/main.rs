use queens::{calc_and_print, print_fields, Fields};

fn main() {
    println!("Unsolved 1 Queen: {}", Fields::default(1));
    println!("Solved 1 Queen: {}", Fields::default(1).get_all_solutions()[0]);
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
