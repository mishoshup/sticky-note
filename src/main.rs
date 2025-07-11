use eframe::{egui, App};
use serde::{Deserialize, Serialize};
use egui::ViewportBuilder;
use serde_json; 
use std::fs;    
use std::path::PathBuf; 

const DATA_FILE_NAME: &str = "sticky_note_data.json";

fn get_data_file_path() -> Option<PathBuf> {
    if let Some(mut path) = dirs::data_dir() {
        path.push("sticky-note-app");

        if let Err(e) = fs::create_dir_all(&path) {
            eprintln!("Error: Failed to create data directory at {:?}: {}", path, e);
            return None;
        }

        path.push(DATA_FILE_NAME);
        Some(path)
    } else {
        eprintln!("Error: Could not determine data directory for the OS.");
        None
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(egui::vec2(300.0, 200.0))
            .with_resizable(true)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "Sticky Note",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct MyApp {
    note_content: String,
}

impl MyApp {
    fn new() -> Self {
        if let Some(path) = get_data_file_path() {
            if path.exists() {
                match fs::read_to_string(&path) {
                    Ok(json_string) => {
                        match serde_json::from_str(&json_string) {
                            Ok(app_state) => {
                                println!("Loaded state from: {:?}", path);
                                return app_state;
                            },
                            Err(e) => eprintln!("Error: Failed to parse JSON from {:?}: {}", path, e),
                        }
                    },
                    Err(e) => eprintln!("Error: Failed to read file {:?}: {}", path, e),
                }
            } else {
                println!("No saved file found at: {:?}", path);
            }
        }
        println!("Starting with default (empty) note.");
        MyApp::default()
    }

    fn save_to_file(&self) {
        if let Some(path) = get_data_file_path() {
            match serde_json::to_string_pretty(self) { 
                Ok(json_string) => {
                    match fs::write(&path, json_string) {
                        Ok(_) => println!("Saved state to: {:?}", path),
                        Err(e) => eprintln!("Error: Failed to write to file {:?}: {}", path, e),
                    }
                },
                Err(e) => eprintln!("Error: Failed to serialize app state for saving: {}", e),
            }
        }
    }
}

impl App for MyApp {
     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.modifiers.command_only() && i.key_pressed(egui::Key::S)) {
            self.save_to_file(); 
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.note_content)
                    .hint_text("Start taking notes here...")
                    .frame(false),
            );
        });
    }
}