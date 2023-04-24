pub mod graphics {
	pub const FRAME_TIME: u64 = 60; // millis
	pub const BUFFER_HEIGHT: usize = 5;
	pub const BUFFER_WIDTH: usize = 14;
	pub fn clear_screen() {
		print!("{e}[2J{e}[1;1H", e = 27 as char);
	}

	pub fn move_pointer(pos: &mut [u32], buffer: &mut [[char; BUFFER_WIDTH]; BUFFER_HEIGHT]) {
		match pos[0] {
			x if x < (BUFFER_HEIGHT) as u32 => {
				if pos[1] < (BUFFER_WIDTH - 1) as u32 {
					pos[1] += 1;
					if x == (BUFFER_HEIGHT - 1) as u32 && pos[1] == (BUFFER_WIDTH - 1) as u32 {
						pos[0] = 0;
						pos[1] = 0;
						clear_buffer(buffer);
					}
				} else {
					pos[0] += 1;
					pos[1] = 0;
				}
			}
			_ => {}
		}
	}

	pub fn print_screen(buffer: &[[char; BUFFER_WIDTH]; BUFFER_HEIGHT]) {
		for &row in buffer {
			for col in row {
				print!("{}", col);
			}
			println!();
		}
	}

	pub fn clear_buffer(buffer: &mut [[char; BUFFER_WIDTH]; BUFFER_HEIGHT]) {
		*buffer = [['▁'; BUFFER_WIDTH]; BUFFER_HEIGHT];
	}

	pub fn new_buffer() -> [[char; BUFFER_WIDTH]; BUFFER_HEIGHT] {
		[['▁'; BUFFER_WIDTH]; BUFFER_HEIGHT]
	}
}
