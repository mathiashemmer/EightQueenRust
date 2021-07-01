use rand::Rng;

pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Copy, Clone)]
pub struct BoardManifest {
    pub queens: [i32; BOARD_SIZE],
	pub collisions: i32
}

impl Default for BoardManifest{
	#[inline]
	fn default() -> Self {
		BoardManifest { 
			queens: [0; BOARD_SIZE],
			collisions: 0 
		}
	}
}

impl BoardManifest{
	pub fn new() -> Self {
		return BoardManifest { 
			queens: [0; BOARD_SIZE],
			collisions: 0 
		};
	}

	pub fn new_random() -> Self{
		let mut new_board = BoardManifest::new();
		
		for queen in new_board.queens.iter_mut(){
			let number = rand::thread_rng().gen_range(0..BOARD_SIZE) as i32; 
			*queen = number;
		}
	
		return new_board;
	}

	pub fn calculate_collisions(&mut self){
		for queen_position_y in 0..BOARD_SIZE {
			let queen_position_x = self.queens[queen_position_y] as i32;
			for target_queen_position_y in (queen_position_y+1)..BOARD_SIZE{
				let target_queen_position_x = self.queens[target_queen_position_y] as i32;
	
				self.collisions += (queen_position_x == target_queen_position_x) as i32;
				let delta_column = (queen_position_x - target_queen_position_x).abs();
				let delta_row = (queen_position_y as i32 - target_queen_position_y as i32).abs();
				self.collisions += (delta_row == delta_column) as i32;
			}
		}
	}
}

pub fn print_board(board: &BoardManifest){
	for row in 0..BOARD_SIZE{
		for column in 0..BOARD_SIZE{
			if column as i32 == board.queens[row]{
				print! ("1 ");
			}else{
				print! ("0 ");
			}
		}
		print!("\n");
	}
}

pub fn print_genes(board: &BoardManifest){
	for row in 0..BOARD_SIZE{
		print!("{},", board.queens[row]);
	}
}


