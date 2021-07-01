use std::time::{Instant};

mod board_manifest;
mod genetics;

const ITERATIONS_MAX: i32 = 2000000;

fn main() {

    let id = 0;

    let now = Instant::now();
    let mut generations : Vec<[board_manifest::BoardManifest; genetics::POPULATION]> = Vec::new();
    let mut boards = genetics::create_first_generation_boards();
    
    let mut current_iteration = 0;
    while current_iteration != ITERATIONS_MAX && boards[0].collisions != 0{
        boards.sort_unstable_by(|a, b| a.collisions.cmp(&b.collisions));
        generations.push(boards);

        boards = genetics::create_next_generation_parallel(&mut boards);

        current_iteration += 1;
    }
    
    board_manifest::print_board(&boards[0]);
    println!("Id: {}, Col: {}, Iterations: {}, Time(ms): {}", id, boards[0].collisions, current_iteration, now.elapsed().as_millis());
    print!("Gene: ");
    board_manifest::print_genes(&boards[0]);
}
