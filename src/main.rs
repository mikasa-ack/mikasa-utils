use clap::{Parser, Subcommand};
use mikasa_utils::command;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    InstantiateGrid {
        #[arg(short, long)]
        dimension: Option<u8>,
        /// A boolean to indicate if the command must be executed or not.
        #[arg(short, long)]
        execute: bool,
    },
}

fn main() {
    let mut grid = create_grid(30, 30);
    insert_cross_pattern(&mut grid);
    display_grid(&grid);
}

fn create_grid(rows: usize, cols: usize) -> Vec<Vec<bool>> {
    vec![vec![false; cols]; rows]
}

fn insert_cross_pattern(grid: &mut Vec<Vec<bool>>) {
    let mid_row = grid.len() / 2;
    let mid_col = grid[0].len() / 2;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if row == mid_row || col == mid_col {
                grid[row][col] = true;
            }
        }
    }
}

fn display_grid(grid: &Vec<Vec<bool>>) {
    println!("{:?}", grid);
    // for row in grid {
    //     for cell in row {
    //         print!("{} ", if *cell { "█" } else { " " });
    //     }
    //     println!();
    // }
}
