#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod note_generator;

use eframe::egui;
use note_generator::{generate_random_note, convert_note_format};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_title("Note Generator - Guitar Training"),
        ..Default::default()
    };

    eframe::run_native(
        "Note Generator",
        options,
        Box::new(|_cc| {
            let mut app = NoteGeneratorApp::default();
            app.current_note = generate_random_note(app.allow_sharp, app.use_solfege);
            Box::new(app)
        }),
    )
}

struct NoteGeneratorApp {
    current_note: String,
    allow_sharp: bool,
    use_solfege: bool,
    timer_enabled: bool,
    timer_interval_seconds: f64,
    last_update_time: f64,
}

impl Default for NoteGeneratorApp {
    fn default() -> Self {
        Self {
            current_note: String::new(),
            allow_sharp: false,
            use_solfege: true, // Classical format by default
            timer_enabled: false,
            timer_interval_seconds: 5.0, // 5 seconds by default
            last_update_time: 0.0,
        }
    }
}

impl eframe::App for NoteGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_time = ctx.input(|i| i.time);
        
        // Initialize last update time if it's the first time
        if self.last_update_time == 0.0 {
            self.last_update_time = current_time;
        }
        
        // Automatic timer management
        if self.timer_enabled {
            let elapsed = current_time - self.last_update_time;
            if elapsed >= self.timer_interval_seconds {
                let new_note = generate_random_note(self.allow_sharp, self.use_solfege);
                if new_note == self.current_note {
                    self.current_note = generate_random_note(self.allow_sharp, self.use_solfege);
                } else {
                    self.current_note = new_note;
                }
                self.last_update_time = current_time;
            }
            // Request continuous refresh to display countdown
            ctx.request_repaint();
        } else {
            // Reset timer when disabled
            self.last_update_time = current_time;
        }
        
        // Detect Space key to generate a new note
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            let new_note = generate_random_note(self.allow_sharp, self.use_solfege);
            if new_note == self.current_note {
                self.current_note = generate_random_note(self.allow_sharp, self.use_solfege);
            } else {
                self.current_note = new_note;
            }
            // Reset timer after manual generation
            self.last_update_time = current_time;
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                
                // Title
                ui.heading("🎸 Note Generator");
                ui.add_space(20.0);
                
                // Display current note
                ui.label(egui::RichText::new("Note to find:")
                    .size(16.0)
                    .color(egui::Color32::GRAY));
                
                ui.add_space(10.0);
                
                // Large note display
                ui.label(egui::RichText::new(&self.current_note)
                    .size(72.0)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 150, 255)));
                
                ui.add_space(30.0);
                
                // Checkbox for sharp mode
                ui.checkbox(&mut self.allow_sharp, "Mode with accidentals (sharps/flats)");
                
                ui.add_space(10.0);
                
                // Toggle for notation format
                let previous_solfege = self.use_solfege;
                ui.checkbox(&mut self.use_solfege, "Classical format (Do Ré Mi...)");
                
                // If format changed, convert current note
                if previous_solfege != self.use_solfege {
                    self.current_note = convert_note_format(&self.current_note, self.use_solfege);
                }
                
                ui.add_space(15.0);
                
                // Timer Section
                ui.separator();
                ui.add_space(10.0);
                
                // Checkbox to enable timer
                ui.checkbox(&mut self.timer_enabled, "Automatic timer");
                
                if self.timer_enabled {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Interval (seconds):");
                        ui.add(egui::Slider::new(&mut self.timer_interval_seconds, 1.0..=30.0)
                            .step_by(0.5)
                            .suffix(" s"));
                    });
                    
                    // Display remaining time
                    let current_time = ctx.input(|i| i.time);
                    let elapsed = current_time - self.last_update_time;
                    let remaining = (self.timer_interval_seconds - elapsed).max(0.0);
                    ui.label(egui::RichText::new(format!("Next note in: {:.1} s", remaining))
                        .size(11.0)
                        .color(egui::Color32::from_rgb(100, 150, 255)));
                }
                
                ui.add_space(15.0);
                
                // Button to generate a new note
                if ui.button(egui::RichText::new("🎲 New Note")
                        .size(18.0))
                    .clicked() {
                      let new_note = generate_random_note(self.allow_sharp, self.use_solfege);
                      if new_note == self.current_note {
                        self.current_note = generate_random_note(self.allow_sharp, self.use_solfege);
                      }else{
                        self.current_note = new_note;
                      }
                }
                
                ui.add_space(20.0);
                
                // Information
                ui.label(egui::RichText::new("Find this note on your guitar fretboard!")
                    .size(12.0)
                    .color(egui::Color32::DARK_GRAY)
                    .italics());
                
                ui.add_space(5.0);
                
                ui.label(egui::RichText::new("Press Space or click the button for a new note")
                    .size(11.0)
                    .color(egui::Color32::DARK_GRAY)
                    .italics());
            });
        });
    }
}
