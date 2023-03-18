use std::collections::LinkedList;
use piston_window::{Context,G2d};
use piston_window::types::Color;
// importing draw_block function from draw.rs
use draw::draw_block;
// snake color [red,green,blue,opacity] of elements.
const SNAKE_COLOR:Color=[0.00,0.80,0.00,1.00];

// snake direction data types
pub enum Direction{
	Up,
	Down,
	Left,
	Right,
}

// implementation of Direction of snake movement 

impl Direction{
	pub fn opposite(&self)->Direction{
		match *self{
			Direction::Up=>Direction::Down,
			Direction::Down=>Direction::Up,
			Direction::Left=>Direction::Right,
			Direction::Right=>Direction::Left,
		}
	}
}

struct Block{
	x:i32,
	y:i32,
}
pub struct Snake{
	direction:Direction,
	body:LinkedList<Block>,
	tail:Option<Block>,

}


