use crate::board_manifest;

use rand::Rng;
use std::mem::{MaybeUninit};
use board_manifest::BoardManifest;
use rayon::prelude::*;

pub const POPULATION: usize = 32;
const SELECTION_FACTOR: f32 = 3.5;
const CROSS_POINT_AT: usize = 0;

pub fn copulate(father: &board_manifest::BoardManifest, mother: &board_manifest::BoardManifest, crosspoint: usize) -> (board_manifest::BoardManifest, board_manifest::BoardManifest){
	let mut child_1 = board_manifest::BoardManifest::new();
	let mut child_2 = board_manifest::BoardManifest::new();

	for queen_index in 0..board_manifest::BOARD_SIZE{
		if queen_index < crosspoint {
			child_1.queens[queen_index] = father.queens[queen_index];
			child_2.queens[queen_index] = mother.queens[queen_index];
		} else {
			child_1.queens[queen_index] = mother.queens[queen_index];
			child_2.queens[queen_index] = father.queens[queen_index];
		}
	}

	mutate(&mut child_1);
	mutate(&mut child_2);
	
	return (child_1, child_2);
}

fn mutate(child: &mut board_manifest::BoardManifest){
	let mutate_first = rand::thread_rng().gen_range(0..101);
	let mutate_last = rand::thread_rng().gen_range(0..101);

	if mutate_first > 90{
		let queen = rand::thread_rng().gen_range(0..board_manifest::BOARD_SIZE) as usize;
		let new_index = rand::thread_rng().gen_range(0..board_manifest::BOARD_SIZE);

		child.queens[queen] = new_index as i32;
	}

	if mutate_last > 98{
		let queen = rand::thread_rng().gen_range(0..board_manifest::BOARD_SIZE) as usize;
		let new_index = rand::thread_rng().gen_range(0..board_manifest::BOARD_SIZE);

		child.queens[queen] = new_index as i32;
	}
}

pub fn create_first_generation_boards() -> [BoardManifest; POPULATION]{
    let mut boards : [MaybeUninit<BoardManifest>; POPULATION] = unsafe { MaybeUninit::uninit().assume_init() };

    for index in 0..POPULATION{
        let mut board = BoardManifest::new_random();
        board.calculate_collisions();
        boards[index] = MaybeUninit::new(board);
    }
    unsafe { std::mem::transmute::<_, [board_manifest::BoardManifest; POPULATION]>(boards) }
}
/*
pub fn create_next_generation(last_generation: &[board_manifest::BoardManifest; POPULATION]) -> [board_manifest::BoardManifest; POPULATION] {
    let mut boards : [MaybeUninit<board_manifest::BoardManifest>; POPULATION]    = unsafe { MaybeUninit::uninit().assume_init() };

    let mut index: usize = 0;
    while index < POPULATION {
        let father_index = (((POPULATION-1) as f32) * ((rand::thread_rng().gen_range(0.0 .. 1.0) as f32).powf(SELECTION_FACTOR))) as usize;
        let mut mother_index = (((POPULATION-1) as f32) * ((rand::thread_rng().gen_range(0.0 .. 1.0) as f32).powf(SELECTION_FACTOR))) as usize;
        while mother_index == father_index{
            mother_index = (((POPULATION-1) as f32) * ((rand::thread_rng().gen_range(0.0 .. 1.0) as f32).powf(SELECTION_FACTOR))) as usize;
        }

        let cross_point;
        if CROSS_POINT_AT > 0{
            cross_point = CROSS_POINT_AT;
        }else{
            cross_point = rand::thread_rng().gen_range(0..board_manifest::BOARD_SIZE)
        }
        
        let (mut child_board_1, mut child_board_2) = copulate(&last_generation[father_index], &last_generation[mother_index], cross_point);
        child_board_1.calculate_collisions();
        child_board_2.calculate_collisions();
        boards[index] = MaybeUninit::new(child_board_1);
        boards[index+1] = MaybeUninit::new(child_board_2);
        index += 2;
    }

    unsafe { std::mem::transmute::<_, [board_manifest::BoardManifest; POPULATION]>(boards) }
}
*/
pub fn create_next_generation_parallel(last_generation: &[board_manifest::BoardManifest; POPULATION]) -> [board_manifest::BoardManifest; POPULATION] {
    let mut boards : [MaybeUninit<board_manifest::BoardManifest>; POPULATION] = unsafe { MaybeUninit::uninit().assume_init() };

    boards.par_chunks_mut(2).for_each(|element| {
        let father_index = (((POPULATION-1) as f32) * (rand::thread_rng().gen_range(0.0 .. 1.0) as f32).powf(SELECTION_FACTOR)) as usize;
        let mut mother_index = (((POPULATION-1) as f32) * (rand::thread_rng().gen_range(0.0 .. 1.0) as f32).powf(SELECTION_FACTOR)) as usize;
        while mother_index == father_index{
            mother_index = ((POPULATION as f32) * (rand::thread_rng().gen_range(0.0 .. 1.0) as f32).powf(SELECTION_FACTOR)) as usize;
        }

        let cross_point;
        if CROSS_POINT_AT > 0{
            cross_point = CROSS_POINT_AT;
        }else{
            cross_point = rand::thread_rng().gen_range(0..board_manifest::BOARD_SIZE)
        }
        
        let (mut child_board_1, mut child_board_2) = copulate(&last_generation[father_index as usize], &last_generation[mother_index], cross_point);
        child_board_1.calculate_collisions();
        child_board_2.calculate_collisions();
        element[0] = MaybeUninit::new(child_board_1);
        element[1] = MaybeUninit::new(child_board_2);
        }
    );
    unsafe { std::mem::transmute::<_, [board_manifest::BoardManifest; POPULATION]>(boards) }
}
