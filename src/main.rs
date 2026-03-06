// Imports all macroquad libaries. 
use macroquad::prelude::*;

// Conf example https://github.com/not-fl3/macroquad/blob/fa486ff74caa8b94b2eeee3cad1da50eb393b53b/examples/window_conf.rs
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Game Version 1".to_string(),
        ..Default::default()
    }
}

// Initiates window and runs specified function asynchronously. 
#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(RED);
        next_frame().await
    }
}