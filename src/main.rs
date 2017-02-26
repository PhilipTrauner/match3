extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

#[macro_use]
extern crate lazy_static;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::*;
use graphics::math::Matrix2d;

use rand::Rng;
use std::mem;


const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const LINE_WIDTH: f64 = 2.5;
const ROWS: i32 = 5;
const COLUMNS: i32 = 5;

lazy_static! {
	pub static ref TILE_SIZE: Size = Size {width: 100.0, height: 100.0};
}


#[derive(Copy, Clone, PartialEq)]
struct Point {
	x: f64,
	y: f64
}

#[derive(Copy, Clone, PartialEq)]
pub struct Size {
	width: f64,
	height: f64
}

pub struct Offset {
	x: f64,
	y: f64
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Tile {
	color: [f32; 4],
	selected: bool,
}

impl Tile {
	fn new(color: [f32; 4]) -> Tile {
		Tile {
			color: color,
			selected: false,
		}
	}

	/*
	fn next_to(&self, tile: &Tile, offset: &Offset) -> bool {
		let x1 = self.position.x + offset.x;
		let y1 = self.position.y + offset.y;		
		
		let x2 = tile.position.x + offset.x;
		let y2 = tile.position.y + offset.y;	

		if x2 - TILE_SIZE.width == x1 || 
			x2 + TILE_SIZE.width == x1 ||
			x2 == x1 {
			if y2 - TILE_SIZE.height == y1 || 
				y2 + TILE_SIZE.height == y1 ||
				y2 == y1 {
				return true;
			}
		}
		return false;
	}
	*/


	fn set_color(&mut self, color: [f32; 4]) {
		self.color = color;
	}

	fn set_selected(&mut self, selected: bool) {
		self.selected = selected;
	}
}

pub struct App {
	gl: GlGraphics,
	tiles: [[Tile; COLUMNS as usize]; ROWS as usize],
	last_clicked_tile_index: Option<[usize; 2]>,
	update_required: bool
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		let ref mut tiles = self.tiles;
		if self.update_required {
			//println!("drawing");
			self.gl.draw(args.viewport(), |c, gl| {
				clear(WHITE, gl);
				let transform = c.transform.trans(
					0.0, 0.0);
				for row in 0..ROWS {
					for column in 0..COLUMNS {
						let tile = tiles[row as usize][column as usize];
						let x = column as f64 * TILE_SIZE.height;
						let y = row as f64 * TILE_SIZE.width;
						rectangle(tile.color, 
							[x, y, 
							TILE_SIZE.width, TILE_SIZE.height], 
							transform, gl);
						if tile.selected {
							line(BLACK, LINE_WIDTH, 
								[x, 
								y + LINE_WIDTH,
								x + TILE_SIZE.width,
								y + LINE_WIDTH], 
								transform, gl);
							line(BLACK, LINE_WIDTH, 
								[x + LINE_WIDTH, 
								y + LINE_WIDTH,
								x + LINE_WIDTH,
								y + TILE_SIZE.height], 
								transform, gl);
							line(BLACK, LINE_WIDTH, 
								[x + TILE_SIZE.width - LINE_WIDTH, 
								y,
								x + TILE_SIZE.width - LINE_WIDTH,
								y + TILE_SIZE.height], 
								transform, gl);
							line(BLACK, LINE_WIDTH, 
								[x, 
								y + TILE_SIZE.height - LINE_WIDTH,
								x + TILE_SIZE.width,
								y + TILE_SIZE.height - LINE_WIDTH], 
								transform, gl);						
						}
					}
				}
			});
			//self.update_required = false;
		}
	}


	fn update(&mut self, args: &UpdateArgs) {
		//println!("update");
		//self.update_required = true;
	}


	fn resize(&mut self, args: &[u32; 2]) {
		//self.update_required = true;
	}

	fn mouse_click(&mut self, click: &Point) {
		let ref mut tiles = self.tiles;

		let mut clicked_tile: Option<[usize; 2]> = None;
		for row in 0..ROWS {
			for column in 0..COLUMNS {
				let x = column as f64 * TILE_SIZE.height;
				let y = row as f64 * TILE_SIZE.width;
				let usize_row = row as usize;
				let usize_column = column as usize;
				if click.x >= x && click.x <= x + TILE_SIZE.width {
					if click.y >= y && click.y <= y + TILE_SIZE.height {
						let tile_index = [usize_row, usize_column];
						if tiles[tile_index[0]][tile_index[1]].selected {
							tiles[tile_index[0]][tile_index[1]].set_selected(false);
						} else {
							tiles[tile_index[0]][tile_index[1]].set_selected(true);
							clicked_tile = Some(tile_index);
							println!("{:?}", tile_index);
						}
					}
				}
			}
		}
		match clicked_tile {
			// No tile selected
			None => {
				// Unselect all tiles
				for row in 0..ROWS as usize {
					for column in 0..COLUMNS {
						tiles[row as usize][column as usize].set_selected(false);
					}
				}
				self.last_clicked_tile_index = None;		
			}
			// Tile selected
			Some(_) => {
				let unwrapped_clicked_tile_index = clicked_tile.unwrap();
				let mut swapped = false;

				for row in 0..ROWS {
					for column in 0..COLUMNS {
						let tile_index = [row as usize, column as usize];
						if tile_index != unwrapped_clicked_tile_index {
							tiles[tile_index[0]][tile_index[1]].set_selected(false);
						} 
					}
				}
				if self.last_clicked_tile_index.is_some() {
					let unwrapped_last_clicked_tile_index = self.last_clicked_tile_index.unwrap();

					if unwrapped_last_clicked_tile_index != unwrapped_clicked_tile_index {
						// Hack to swap positions
						// Same row
						if unwrapped_clicked_tile_index[0] == unwrapped_last_clicked_tile_index[0] {
							println!("same row");
							let row = unwrapped_clicked_tile_index[0] as usize;
							// Column 1 has to be the smaller one
							let mut column_1: usize = 0;
							let mut column_2: usize = 0;
							if unwrapped_clicked_tile_index[1] < unwrapped_last_clicked_tile_index[1] {
								column_1 = unwrapped_clicked_tile_index[1];
								column_2 = unwrapped_last_clicked_tile_index[1];
							} else {
								column_1 = unwrapped_last_clicked_tile_index[1];
								column_2 = unwrapped_clicked_tile_index[1];							
							}
							if !((column_1 + 1) >= COLUMNS as usize) {
								let (x, y) = tiles[row].split_at_mut(column_1 + 1);
								let x_length = x.len();
								mem::swap(&mut x[column_1], &mut y[column_2 - x_length]);
								x[column_1].set_selected(false);
								y[column_2 - x_length].set_selected(false);
							} else {
								let (x, y) = tiles[row].split_at_mut(column_1 - 1);
								let x_length = x.len();
								mem::swap(&mut x[column_2], &mut y[column_1 - x_length]);
								x[column_2].set_selected(false);
								y[column_1 - x_length].set_selected(false);
							}
						// Different row
						} else {
							println!("different row (nyi)");
							// Row 1 has to be the smaller one
							let mut row_1: usize = 0;
							let mut row_2: usize = 0;
							if unwrapped_clicked_tile_index[0] < unwrapped_last_clicked_tile_index[0] {
								row_1 = unwrapped_clicked_tile_index[0];
								row_2 = unwrapped_last_clicked_tile_index[0];
							} else {
								row_1 = unwrapped_last_clicked_tile_index[0];
								row_2 = unwrapped_clicked_tile_index[0];							
							}
							if !((row_1 + 1) >= ROWS as usize) {
								let (x, y) = tiles.split_at_mut(row_1 + 1);

							}
						}
						self.last_clicked_tile_index = None;
						swapped = true;
						/*
						let ref temp_tile = tiles[unwrapped_last_clicked_tile_index[0]][unwrapped_last_clicked_tile_index[1]];
						tiles[unwrapped_clicked_tile_index[0]][unwrapped_clicked_tile_index[1]] = *temp_tile;
						*/
						/*
						let clicked_tile_clone = self.tiles[unwrapped_clicked_tile].clone();
						let last_clicked_tile_clone = self.tiles[unwrapped_last_clicked_tile].clone();

						self.tiles[unwrapped_clicked_tile].set_selected(false);
						self.tiles[unwrapped_last_clicked_tile].set_selected(false);
						self.tiles[unwrapped_clicked_tile].set_position(last_clicked_tile_clone.position);
						self.tiles[unwrapped_last_clicked_tile].set_position(clicked_tile_clone.position);

						self.last_clicked_tile = None;
						swapped = true;
						*/
					}
				}
				if !swapped {
					self.last_clicked_tile_index = Some(unwrapped_clicked_tile_index);
				}
			}
		}
	}
}

fn main() {
	// Change this to OpenGL::V2_1 if not working.
	let opengl = OpenGL::V3_2;
	let mut cursor = Point {x: 0.0, y: 0.0};
	// Create an Glutin window.
	let mut window: Window = WindowSettings::new(
			"match3",
			[500, 500]
		)
		.opengl(opengl)
		.exit_on_esc(true)
		.decorated(true)
		.build()
		.unwrap();

	let colors = vec![RED, GREEN, BLUE];

	let mut tiles: [[Tile; COLUMNS as usize]; ROWS as usize] = [
		[Tile::new(WHITE); COLUMNS as usize]; ROWS as usize
	];


	for row in 0..ROWS {
		let usize_row = row as usize;
		for column in 0..COLUMNS {
			let usize_column = column as usize;
			tiles[usize_row][usize_column] = 
				Tile::new(*rand::thread_rng().choose(&colors).unwrap());
		}
	}


	let mut app = App {
		gl: GlGraphics::new(opengl),
		tiles: tiles,
		last_clicked_tile_index: None,
		update_required: true
	};

	let mut events = Events::new(EventSettings::new());
	while let Some(e) = events.next(&mut window) {
		if let Some(r) = e.render_args() {
			app.render(&r);
		}
		
		if let Some(u) = e.update_args() {
			app.update(&u);
		}

		if let Some(res) = e.resize_args() {
			app.resize(&res);
		}

		if let Some(Button::Mouse(button)) = e.press_args() {
			app.mouse_click(&cursor);
		}
		e.mouse_cursor(|x, y| {
			cursor = Point {x: x, y: y};
		}); 
	}
}