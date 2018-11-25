extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

struct Block {
    shape: [[u8; 4]; 4],
    position: [[[u8; 2]; 4]; 4],
    colider: [i8; 3]
}

struct Game {
    score: u64,
    gl: GlGraphics,
    well: [[i8; 10]; 22],
    curr_block: Block
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Tetrust",
        [800, 600]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut game = Game::new(GlGraphics::new(opengl));
    
    let mut events = Events::new(EventSettings::new()).ups(6);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render();
        }
        if let Some(u) = e.update_args() {
            game.update();
        }
    }
}

impl Game {
    pub fn new(_gl: GlGraphics) -> Self {

        return Game {
            score: 0,
            gl: _gl,
            well: [[0; 10]; 22],
            curr_block: Block::new() 
        }
    }

    #[cfg(debug_assertions)]
    fn render(&mut self) {
        print!("{}[2J", 27 as char);
        let mut screen = self.well.clone();
        for i in 0..4 {
            for j in 0..4 {
                let pos = self.curr_block.position;
                if self.curr_block.shape[i][j] > 0 {
                    screen
                        [pos[i][j][0] as usize]
                        [pos[i][j][1] as usize] = self.curr_block.shape[i][j] as i8;
                }
            }
        }
        for r in screen.iter() {
            println!("{:?}", r);
        }
    }

    fn update(&mut self) {
        if self.curr_block.colider[1] + 1 <= 21 {
            for i in 0..4 {
                for j in 0..4 {
                    self.curr_block.position[i][j][0] += 1;
                }
            }
            self.curr_block.colider[1] += 1;
        }
    }
}

impl Block {
    pub fn new () -> Self {
        use rand::Rng;
        use rand::thread_rng;

        let mut pos: [[[u8; 2]; 4]; 4] = [[[0; 2]; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                pos[i][j] = [i as u8, j as u8];
            }
        }

        let mut r = thread_rng();
        let s = block_type[r.gen_range(0, 7)];
        
        let mut right_col: i8 = 0;
        let mut down_col: i8 = 0;
        let mut left_col: i8 = -1;

        for i in 0..4 {
            for j in 0..4 {
                if s[i][j] > 0 {
                    if left_col == -1 { left_col = j as i8; }
                    if right_col < i as i8 { right_col = j as i8; }
                    if down_col < j as i8 { down_col = i as i8; }
                }
            }
        }
        
        Block {
            shape: s,
            position: pos,
            colider: [left_col, down_col, right_col],
        }
    }

    fn rotate(&mut self, n: u16) {
        for _ in 1..n {
            for i in 0..4 { 
                for j in 0..4 {
                    let mut tmp = self.shape[i][j];
                    self.shape[i][j] = self.shape[j][i];
                    self.shape[j][i] = tmp;
                }
            }
        }
    }
}

const i_block: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [0, 0, 0, 0],
    [0, 0, 0, 0]
];

const o_block: [[u8; 4]; 4] = [
    [0, 2, 2, 0],
    [0, 2, 2, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
];

const t_block: [[u8; 4]; 4] = [
    [0, 3, 0, 0],
    [3, 3, 3, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]
];

const s_block: [[u8; 4]; 4] = [
    [0, 4, 4, 0],
    [4, 4, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]
];

const z_block: [[u8; 4]; 4] = [
    [5, 5, 0, 0],
    [0, 5, 5, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]
];

const j_block: [[u8; 4]; 4] = [
    [0, 6, 0, 0],
    [0, 6, 0, 0],
    [6, 6, 0, 0],
    [0, 0, 0, 0]
];

const l_block: [[u8; 4]; 4] = [
    [7, 0, 0, 0],
    [7, 0, 0, 0],
    [7, 7, 0, 0],
    [0, 0, 0, 0]
];

const block_type: [[[u8; 4]; 4]; 7] = [
    i_block,
    o_block,
    t_block,
    s_block,
    z_block,
    j_block,
    l_block
];

