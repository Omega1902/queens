use clap::Parser;
use queens::{calc_and_print, print_fields, Fields};

/// Solves the queens puzzle
#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    /// How many queens to use
    #[arg(default_value_t = 8)]
    size: u8,

    /// Prints the found solutions
    #[arg(short, long)]
    print: bool,
}

fn main() {
    let args = Args::parse();

    if args.size == 132 {
        //debug stuff
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

        return;
    }

    calc_and_print(args.size, args.print);
}
