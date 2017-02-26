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

const USIZE_ROWS: usize = ROWS as usize;
const USIZE_COLUMNS: usize = COLUMNS as usize;

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

	fn set_color(&mut self, color: [f32; 4]) {
		self.color = color;
	}

	fn set_selected(&mut self, selected: bool) {
		self.selected = selected;
	}
}

pub struct App {
	gl: GlGraphics,
	tiles: [[Tile; USIZE_COLUMNS]; USIZE_ROWS],
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
							println!("clicked {:?}", tile_index);
						}
					}
				}
			}
		}
		match clicked_tile {
			// No tile selected
			None => {
				// Unselect all tiles
				for row in 0..USIZE_ROWS {
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
						// Restrict movement in + shape
						let mut row_1 = unwrapped_clicked_tile_index[0];
						let mut column_1 = unwrapped_clicked_tile_index[1];
						let mut row_2 = unwrapped_last_clicked_tile_index[0];
						let mut column_2 = unwrapped_last_clicked_tile_index[1];
						
						let mut allowed_move = true;

						if row_2 == 0 {
							if row_1 == 1 && column_1 == 0 {
								allowed_move = true;
							} else if row_1 == 0 && column_1 == 1 {
								allowed_move = true;
							}
						} else if row_2 == USIZE_ROWS {
							if row_1 == USIZE_ROWS - 1 {
								allowed_move = true;
							}
							if column_1 == USIZE_ROWS - 1 {
								allowed_move = true;
							}
						} else if column_2 == 0 {
							if column_1 == 1 {
								allowed_move = true;
							}
						} else if column_2 == USIZE_COLUMNS {
							if column_1 == USIZE_COLUMNS - 1 {
								allowed_move = true;
							}
						}



						if allowed_move {
						// Hack to swap positions (only one array element can be borrowed mutably)
						// Same row
							if unwrapped_clicked_tile_index[0] == unwrapped_last_clicked_tile_index[0] {
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
								let (x, y) = tiles[row].split_at_mut(column_1 + 1);
								mem::swap(&mut x[column_1], &mut y[column_2 - (column_1 + 1)]);
								x[column_1].set_selected(false);
								y[column_2 - (column_1 + 1)].set_selected(false);
								println!("swapped (same row) [{}, {}] [{}, {}]", row, column_1, row, column_2);
							// Different row
							} else {
								// Row 1 has to be the smaller one
								let mut row_1: usize = 0;
								let mut row_2: usize = 0;
								let mut column_1: usize = 0;
								let mut column_2: usize = 0;
								if unwrapped_clicked_tile_index[0] < unwrapped_last_clicked_tile_index[0] {
									row_1 = unwrapped_clicked_tile_index[0];
									row_2 = unwrapped_last_clicked_tile_index[0];
									column_1 = unwrapped_clicked_tile_index[1];
									column_2 = unwrapped_last_clicked_tile_index[1];
								} else {
									row_1 = unwrapped_last_clicked_tile_index[0];
									row_2 = unwrapped_clicked_tile_index[0];
									column_1 = unwrapped_last_clicked_tile_index[1];
									column_2 = unwrapped_clicked_tile_index[1];							
								}
								let (x, y) = tiles.split_at_mut(row_1 + 1);
								mem::swap(&mut x[row_1][column_1], &mut y[row_2 - (row_1 + 1)][column_2]);
								x[row_1][column_1].set_selected(false);
								y[row_2 - (row_1 + 1)][column_2].set_selected(false);
								println!("swapped (different row) [{}, {}] [{}, {}]", row_1, column_1, row_2, column_2);
							}
							self.last_clicked_tile_index = None;
							swapped = true;
							let mut last_color: Option<[f32; 4]> = None; 
							let mut color_count = 0;
							/*
							for row in 0..USIZE_ROWS {
								for column in 0..COLUMNS {
									if last_color.is_none() {
										last_color = tiles[row][column].color;
										color_count += 1;
									}
									if last_color.is_some() {
										if last_color == tiles[row][column].color {
											color_count += 1;
										}
									}
									if color_count >= 3 {

									}
								}
							}
							for column in 0..USIZE_COLUMNS {
								for row in 0..ROWS {
									
								}
							}
							*/					
						}
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

	let mut tiles: [[Tile; USIZE_COLUMNS]; USIZE_ROWS] = [
		[Tile::new(WHITE); USIZE_COLUMNS]; USIZE_ROWS
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