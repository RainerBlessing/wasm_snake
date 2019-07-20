use std::time::Duration;

use std::path::Path;

use crate::constants::Type;
use crate::constants::PlayField;

use crate::constants::Keyboard;

use crate::constants::GameState;
use crate::events::SnakeEvent;
use crate::snake::Snake;


use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    HtmlCanvasElement,
    KeyboardEvent,
};
use wasm_bindgen::prelude::*;
use std::f64;
use std::{rc::Rc, cell::RefCell, cell::Cell};

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct JSCanvas {
    context: web_sys::CanvasRenderingContext2d,
    canvas: web_sys::HtmlCanvasElement,
    frame_delay: i32,
    ticks: i32,
    game_state: GameState,
}

static mut SNAKE: Option<Snake> = None;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const SNAKE_WIDTH: u32 = WIDTH / 40;
const M: usize = 40;
const N: usize = 40;


trait Canvas {}

impl Canvas for JSCanvas {}


impl JSCanvas {
    pub fn new() -> JSCanvas {

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        JSCanvas {
            canvas,
            context,
            frame_delay: 1000 / 60,
            ticks: 0,
            game_state: GameState::GameOver,
        }
    }

    pub fn create_score_text(&mut self, text: &str) -> () {
////             render a surface, and convert it to a texture bound to the canvas
//        let surface = self.font.render(text)
//            .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
//        let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
//
//
//        let TextureQuery { width, height, .. } = texture.query();
//// If the example text is too big for the screen, downscale it (and center irregardless)
//        let padding = 64;
//        let target = self.get_centered_rect(width, height, WIDTH - padding, HEIGHT - padding);
//        self.canvas.copy(&texture, None, Some(target)).unwrap();
    }


    pub fn draw(&mut self, grid: [[PlayField; 40]; 40]) -> () {
        self.context.stroke();
        for i in 0..M {
            for j in 0..N {
                self.context.begin_path();

                match grid[i][j].field_type {
                    Type::Empty => {
                        self.color_black();
                    }
                    Type::Wall => {
                        self.color_white();
                    }
                    Type::Snake => {
                        self.color_green();
                    }
                    Type::Apple => {
                        self.color_red();
                    }
                }

                self.context.fill_rect(grid[i][j].x as f64, grid[i][j].y as f64, SNAKE_WIDTH as f64, SNAKE_WIDTH as f64);
                self.context.close_path();
                self.context.stroke();
            }
        }
    }

    fn color_red(&mut self) {
        self.context.set_fill_style(&JsValue::from_str("#FF0000"));
    }

    fn color_green(&mut self) {
        self.context.set_fill_style(&JsValue::from_str("#00FF00"));
    }

    fn color_white(&mut self) {
        self.context.set_fill_style(&JsValue::from_str("#FFF"));
    }

    fn color_black(&mut self) {
        self.context.set_fill_style(&JsValue::from_str("#000"));
    }


    fn parse_event(&mut self, snake_event: SnakeEvent) -> () {
        let snake = unsafe { SNAKE.as_mut().unwrap() };
        let key = snake_event.get_key();
        match key {
            Keyboard::Up => snake.move_up(),
            Keyboard::Down =>snake.move_down(),
            Keyboard::Left => snake.move_left(),
            Keyboard::Right => snake.move_right(),
            _ => {}
        }
    }

    pub fn update(&mut self) -> () {
        let snake = unsafe { SNAKE.as_mut().unwrap() };

        let grid = snake.draw_elements();
        self.draw(grid);
        match self.game_state {
            GameState::Play => {
                self.game_state = snake.play_state();
            }
            _ => {}
        }
    }

    pub fn start(&mut self) -> () {
        unsafe {
            SNAKE = Some(Snake {
                game_state: crate::constants::GameState::Play,
                score: 0,
                velocity_y: 0,
                velocity_x: 0,
                snake_vec: Vec::new(),
                apple_vec: Vec::new(),
                wrap: false,
                rng: rand::thread_rng(),
                grid: [[PlayField { field_type: Type::Empty, x: 0, y: 0 }; N]; M],
            });
        }

        self.game_state = GameState::Play;
        let window = web_sys::window().unwrap();
        let snake = unsafe { SNAKE.as_mut().unwrap() };
        snake.setup_board();

        let onkeydown_handler = Closure::wrap(Box::new(|event: KeyboardEvent| {
                console_log!("key {}", event.key_code());
                on_key(event.key_code(), true);
            }) as Box<FnMut(KeyboardEvent)>);

            window.set_onkeydown(Some(onkeydown_handler.as_ref().unchecked_ref()));
            onkeydown_handler.forget();



//        let mut event_pump = self.sdl_context.event_pump().unwrap();
//
//        let grid = snake.draw_elements();
//        self.draw(grid);


//        'running: loop {
//            self.start_loop();
//
//            self.clear();
//
//            let grid = snake.draw_elements();
//
//            self.draw(grid);
//
//            for sdl_event in event_pump.poll_iter() {
//                match game_state {
//                    GameState::Play => {
//                        match sdl_event {
//                            Event::Quit { .. } |
//                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                                break 'running;
//                            }
//
//                            _ => {
//                                self.parse_event(&mut snake, SnakeEvent::new(sdl_event));
//                            }
//                        }
//                    }
//                    GameState::GameOver => {
//                        match sdl_event {
//                            Event::Quit { .. } |
//                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                                break 'running;
//                            }
//                            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
//                                snake.setup_board();
//                                game_state = GameState::Play;
//                            }
//                            _ => {}
//                        }
//                    }
//                }
//            }

//            match game_state {
//                GameState::Play => {
//                    game_state = snake.play_state();
//                }
//                _ => {}
//            }
//
//            self.create_score_text(snake.score.to_string().as_str());
//
//            self.present();
//
//            self.loop_delay()
//        }
    }

}


pub fn on_key(key: u32, state: bool) {
    const KEY_UP: u32 = 38;
    const KEY_DOWN: u32 = 40;
    const KEY_LEFT: u32 = 37;
    const KEY_RIGHT: u32 = 39;

    let snake = unsafe { SNAKE.as_mut().unwrap() };

    match key {
        KEY_UP => snake.move_up(),
        KEY_DOWN => snake.move_down(),
        KEY_LEFT => snake.move_left(),
        KEY_RIGHT => snake.move_right(),
        _ => ()
    };
}