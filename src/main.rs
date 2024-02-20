#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui_macroquad::*;
use macroquad::prelude::*;
use denlibs::random::{Direction, random_direction};
use mouse_rs::Mouse;

const DELAY: f64 = 5.0;

fn conf() -> Conf {
    let mut title = String::from("Mouse mover by -=DeN=- v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: 350,
        window_height: 90,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mouse = Mouse::new();
    let mut time_now: f64 = get_time();
    

    loop {
        clear_background(BLACK);

        

        egui_macroquad::ui(|ctx| {
            egui::CentralPanel::default().show(&ctx, |ui| {
                if ui.button(egui::RichText::new("Quit").color(egui::Color32::RED)).clicked() {
                    std::process::exit(0x0100);
                }
            });
        });

        egui_macroquad::draw();

        if get_time() - time_now > DELAY {
            let cur_mouse_position = mouse_position::mouse_position::Mouse::get_mouse_position();
            match cur_mouse_position {
                mouse_position::mouse_position::Mouse::Position { mut x, mut y } => {
                    match random_direction().unwrap() {
                        Direction::Up => {y -= 1},
                        Direction::Down => {y += 1},
                        Direction::Left => {x -= 1},
                        Direction::Right => {x += 1},
                    };
                    mouse.move_to(x+1, y+1).expect("Unable to move mouse");
                },
                mouse_position::mouse_position::Mouse::Error => println!("Error getting mouse position"),
            }

            time_now = get_time();
        }

        next_frame().await
    }
}

