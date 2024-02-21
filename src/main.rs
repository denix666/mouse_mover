#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};
use eframe::egui;
use denlibs::random::{Direction, random_direction};
use enigo::*;

const DELAY: u128 = 5000;

fn get_time() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn load_icon() -> eframe::IconData {
    eframe::IconData::try_from_png_bytes(&include_bytes!("../assets/icon.png")[..]).unwrap()
}

fn main() {
    let mut time_now = get_time();
    let mut active = true;
    let mut mouse = Enigo::new();
    
    let mut title = String::from("Mouse mover v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 80.0)),
        resizable: false,
        default_theme: eframe::Theme::Dark,
        icon_data: Some(load_icon()),
        ..Default::default()
    };

    eframe::run_simple_native(&title, options, move |ctx, _frame| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut active, "ativated");
            });
            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui.button(egui::RichText::new("Quit").color(egui::Color32::RED)).clicked() {
                    std::process::exit(0x0100);
                }
            });
           
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("By -=DeN=-").color(egui::Color32::WHITE));
            });

            if active {
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
                            mouse.mouse_move_to(x, y);
                            mouse.key_up(Key::Control);
                        },
                        mouse_position::mouse_position::Mouse::Error => println!("Error getting mouse position"),
                    }

                    time_now = get_time();
                }
            }
        });
        
        ctx.request_repaint();
    }).unwrap();
}