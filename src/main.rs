use clap::Parser;
use game_of_life::*;
use std::println;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // width of the grid
    #[arg(short, long, default_value_t = 40)]
    width: usize,

    #[arg(long, default_value_t = 40)]
    height: usize,

    #[arg(short, long, default_value_t = 0.2)]
    probability: f64,
}

fn main() {
    let args = Args::parse();

    let mut grid = Grid::new(args.width, args.height, args.probability);

    loop {
        grid.print_grid();
        grid.update_grid();
        sleep(Duration::from_millis(100));
        println!("\x1B[{}A", args.height + 1); // Move the cursor up to redraw the grid
    }
}
