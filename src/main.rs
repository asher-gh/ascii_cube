use ascii_cube::graphics::{
	clear_screen, move_pointer, print_screen, BUFFER_HEIGHT, BUFFER_WIDTH, FRAME_TIME,
};

/// Drawing an ASCII cube
///         +---------------+
///         |               |
///         |               |
///         |               |
///         |               |
///         |               |
///         |               |
///         +---------------+

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
