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
    shape: [[u8; 4]; 4]
}

struct Game {
    score: u64,
    gl: GlGraphics,
    well: [[i8; 10]; 22],
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
    
    let mut events = Events::new(EventSettings::new()).ups(30);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render();
        }
    }
}

impl Game {
    pub fn new(_gl: GlGraphics) -> Self {

        return Game {
            score: 0,
            gl: _gl,
            well: [[0; 10]; 22]
        }
    }

    #[cfg(debug_assertions)]
    fn render(&mut self) {
        print!("{}[2J", 27 as char);  
        for r in self.well.iter() {
            println!("{:?}", r);
        }
    }
}

impl Block {
    fn rotate(&mut self, n: u16) {
        for _x in 1..n {
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
    
