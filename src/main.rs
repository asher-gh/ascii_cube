/// Drawing an ASCII cube
///         +---------------+
///         |               |
///         |               |
///         |               |
///         |               |
///         |               |
///         |               |
///         +---------------+
const FRAME_TIME: u64 = 60; // millis
const BUFFER_HEIGHT: usize = 5;
const BUFFER_WIDTH: usize = 14;

fn main() {
	// define the screen as a 2D array of chars
	let mut buffer = [['▁'; BUFFER_WIDTH]; BUFFER_HEIGHT];
	let mut pos = [0u32; 2];

	loop {
		buffer[pos[0] as usize][pos[1] as usize] = '█';
		clear_screen();
		print_screen(&buffer);
		dbg!(pos);
		std::thread::sleep(std::time::Duration::from_millis(FRAME_TIME));
		// logic goes here
		move_pointer(&mut pos, &mut buffer);
	}
}

fn clear_screen() {
	print!("{e}[2J{e}[1;1H", e = 27 as char);
}

fn move_pointer(pos: &mut [u32], buffer: &mut [[char; BUFFER_WIDTH]; BUFFER_HEIGHT]) {
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

fn print_screen(buffer: &[[char; BUFFER_WIDTH]; BUFFER_HEIGHT]) {
	for &row in buffer {
		for col in row {
			print!("{}", col);
		}
		println!();
	}
}

fn clear_buffer(buffer: &mut [[char; BUFFER_WIDTH]; BUFFER_HEIGHT]) {
	*buffer = [['▁'; BUFFER_WIDTH]; BUFFER_HEIGHT];
}
