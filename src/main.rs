use bracket_lib::prelude::*;

enum GameMode {
	Menu,
	Playing,
	Pause,
	End,
}

const SCREEN_WIDTH: i32 = 80;
#[allow(dead_code)]
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 35.0;
const FLOOR: i32 = 40;
const PLAYER_COLUMN: i32 = 10;

struct Player {
	x: i32,
	y: i32,
	velocity: i32,
}

impl Player {
	fn new(x: i32, y: i32) -> Self {
		Player { x, y, velocity: 0 }
	}

	fn render(&mut self, ctx: &mut BTerm, color: (u8, u8, u8)) {
		ctx.set(PLAYER_COLUMN, self.y, RGB::named(ORANGE1), color, to_cp437('&'));
	}

	fn gravity_and_move(&mut self) {
		if self.velocity < 10 && self.y < FLOOR {
			self.velocity += 1;
		}
		self.y += self.velocity as i32;
		self.x += 1;
		if self.y > FLOOR {
			self.y = FLOOR;
			self.velocity = 0;
		}
	}

	fn jump(&mut self) {
		self.velocity = -3;
	}
}

struct Obstacle {
	x: i32,
	y: i32,
	velocity: f32,
	symbol: char,
	color: (u8, u8, u8),
}

impl Obstacle {
	fn new(x: i32, score: i32) -> Self {
		let mut random = RandomNumberGenerator::new();
		let x_diff = random.range(0, SCREEN_WIDTH / 2);
		let mut height = random.range(FLOOR - 5, FLOOR + 1);
		let mut velocity = random.range(-1.5, score as f32 * 0.02);
		if velocity < 0.0 {
			velocity = 0.0;
			height = FLOOR;
		}
		Obstacle {
			x: x + x_diff,
			y: height,
			velocity,
			symbol: if velocity > 0.0 { '{' } else { 'f' },
			color: if velocity > 0.0 { RED } else { GREEN },
		}
	}

	fn render(&mut self, ctx: &mut BTerm, player_x: i32, color: (u8, u8, u8)) {
		self.x -= self.velocity as i32;
		let screen_x = self.x - player_x;
		ctx.set(screen_x, self.y, self.color, color, to_cp437(self.symbol));
	}

	fn hit_obstacle(&mut self, player: &Player) -> bool {
		player.x == (self.x - PLAYER_COLUMN) && player.y == self.y
	}
}

struct State {
	player: Player,
	frame_time: f32,
	obstacles: Vec<Obstacle>,
	mode: GameMode,
	score: i32,
}

impl State {
	#[allow(clippy::vec_init_then_push)]
	fn new() -> Self {
		let mut obstacles = Vec::new();
		obstacles.push(Obstacle::new(SCREEN_WIDTH, 0));
		State {
			player: Player::new(PLAYER_COLUMN, FLOOR),
			frame_time: 0.0,
			obstacles,
			mode: GameMode::Menu,
			score: 0,
		}
	}

	// darken the sky as time goes on
	fn sky(&mut self) -> (u8, u8, u8) {
		let modscore = self.score % 50;
		match modscore {
			0..=10 => WHITE,
			11..=20 => LIGHT_GRAY,
			21..=30 => GRAY,
			31..=40 => DARK_GRAY,
			_ => BLACK,
		}
	}

	fn play(&mut self, ctx: &mut BTerm) {
		let color = self.sky();
		ctx.cls_bg(color);
		for i in 0..SCREEN_WIDTH {
			ctx.set(i, FLOOR + 1, RGB::named(DARK_GREEN), color, to_cp437('-'));
		}
		self.frame_time += ctx.frame_time_ms;
		if self.frame_time > FRAME_DURATION {
			self.frame_time = 0.0;
			self.player.gravity_and_move();
		}
		if let Some(key) = ctx.key {
			match key {
				// pause
				VirtualKeyCode::P => self.mode = GameMode::Pause,
				// jump
				VirtualKeyCode::Space => {
					if self.player.y == FLOOR {
						self.player.jump();
					}
				},
				// jump
				VirtualKeyCode::Up => {
					if self.player.y == FLOOR {
						self.player.jump();
					}
				},
				// exit
				VirtualKeyCode::Escape => ctx.quitting = true,
				VirtualKeyCode::Q => ctx.quitting = true,
				_ => {},
			}
		}
		self.player.render(ctx, color);
		let len = self.obstacles.len();
		for obstacle in &mut self.obstacles {
			obstacle.render(ctx, self.player.x, color);
			if obstacle.hit_obstacle(&self.player) {
				self.mode = GameMode::End;
			}
		}
		let diff = self.player.x - 5;
		self.obstacles.retain(|o| o.x > diff);
		let newlen = self.obstacles.len();
		let newscore = len - newlen;
		self.score += newscore as i32;

		if (self.obstacles[newlen - 1].x - self.player.x) < (SCREEN_WIDTH * 9 / 10) {
			self.obstacles.push(Obstacle::new(self.player.x + SCREEN_WIDTH, self.score));
		}
		// display score && onscreen obstacles
		ctx.print_color(0, 1, RGB::named(ORANGE1), color, &format!("Score: {}", self.score));
		ctx.print_color(0, 2, RGB::named(ORANGE1), color, &format!("Obstacles: {}", newlen));
	}

	#[allow(clippy::vec_init_then_push)]
	// start AND restart the game
	fn restart(&mut self) {
		self.player = Player::new(PLAYER_COLUMN, FLOOR);
		self.frame_time = 0.0;
		self.mode = GameMode::Playing;
		let mut obstacles = Vec::new();
		obstacles.push(Obstacle::new(SCREEN_WIDTH, 0));
		self.obstacles = obstacles;
		self.score = 0;
	}

	fn main_menu(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		ctx.print_centered(5, "Welcome to Dino_rs");
		ctx.print_centered(8, "( P || Space ) Play Game");
		ctx.print_centered(10, "( Q || Esc ) Quit Game");
		if let Some(key) = ctx.key {
			match key {
				// start game
				VirtualKeyCode::P => self.restart(),
				VirtualKeyCode::Space => self.restart(),
				// exit
				VirtualKeyCode::Q => ctx.quitting = true,
				VirtualKeyCode::C => ctx.quitting = true,
				VirtualKeyCode::Escape => ctx.quitting = true,
				_ => {},
			}
		}
	}

	fn dead(&mut self, ctx: &mut BTerm) {
		// clear screen
		ctx.cls();
		ctx.print_centered(3, "DEAD");
		ctx.print_centered(6, &format!("You earned {} points", self.score));
		ctx.print_centered(8, "( P || Space ) Play Again");
		ctx.print_centered(10, "( Q || Esc ) Quit Game");
		if let Some(key) = ctx.key {
			match key {
				// restart
				VirtualKeyCode::P => self.restart(),
				VirtualKeyCode::Space => self.restart(),
				// exit
				VirtualKeyCode::Q => ctx.quitting = true,
				VirtualKeyCode::C => ctx.quitting = true,
				VirtualKeyCode::Escape => ctx.quitting = true,
				_ => {},
			}
		}
	}

	fn pause(&mut self, ctx: &mut BTerm) {
		self.dead(ctx);
	}
}

impl GameState for State {
	fn tick(&mut self, ctx: &mut BTerm) {
		match self.mode {
			GameMode::Menu => self.main_menu(ctx),
			GameMode::End => self.dead(ctx),
			GameMode::Playing => self.play(ctx),
			GameMode::Pause => self.pause(ctx),
		}
	}
}

fn main() -> BError {
	let context = BTermBuilder::simple80x50()
		.with_title("Dino_rs")
		// important for font size
		.with_automatic_console_resize(false)
		.with_fitscreen(true)
		.with_gutter(8)
		.build()?;
	main_loop(context, State::new())
}
