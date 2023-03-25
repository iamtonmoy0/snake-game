use  piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng,Rng};
use snake::{Direction,Snake};
use draw::{draw_block,draw_rectangle};

//color 
const FOOD_COLOR:Color=[0.80,0.00,0.00,1.0]; //snake food color
const BORDER_COLOR:Color=[0.00,0.00,0.00,1.0]; //snake board border color
const GAME_OVER:Color=[0.90,0.00,0.00,0.5]; //when the game is over the color will show

//time
const MOVING_PERIOD:f64=0.1;
const RESTART_TIME:f64=1.0;

//structure
pub struct Game{
	snake:Snake,
	food_exists:bool,
	food_x:i32,
	food_y:i32,
	width:i32,
	height:i32,
	game_over:bool,
	waiting_time:f64,
}
//impl
impl Game{
	pub fn new(width:i32,height:i32)->Game{
		Game{
			snake:Snake::new(2,2),
			food_exists:true,
			food_x:6,
			food_y:4,
			width,
			height,
			game_over:false,
			waiting_time:0.0,

		}
	}
pub fn key_pressed(&mut self,key:Key){
	if self.game_over{
		return;
	}

	let dir= match key{
		Key::Up=>Some(Direction::Up),
		Key::Down=>Some(Direction::Down),
		Key::Left=>Some(Direction::Left),
		Key::Right=>Some(Direction::Right),
		_=>None
	};

	if dir.unwrap()== self.snake.head_direction().opposite(){
		return;
	}

	self.update_snake(dir);
}
pub fn draw(&self,con:&Context,g:&mut G2d){


	
}


}