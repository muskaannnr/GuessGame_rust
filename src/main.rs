// src/main.rs
use eframe::{egui, Frame, NativeOptions};
use rand::Rng;
use std::cmp::Ordering;

struct GuessApp {
    secret: u32,
    input: String,
    message: String,
    attempts: u32,
    max_range: u32,
    won: bool,
}

impl GuessApp {
    fn new_game(&mut self) {
        self.secret = rand::thread_rng().gen_range(1..=self.max_range);
        self.input.clear();
        self.message = format!("Guess a number between 1 and {}.", self.max_range);
        self.attempts = 0;
        self.won = false;
    }

    fn check_guess(&mut self) {
        match self.input.trim().parse::<u32>() {
            Ok(guess) if (1..=self.max_range).contains(&guess) => {
                self.attempts += 1;
                match guess.cmp(&self.secret) {
                    Ordering::Equal => {
                        self.message = format!("🎉 Correct! You guessed it in {} tries.", self.attempts);
                        self.won = true;
                    }
                    Ordering::Greater => {
                        self.message = "Too high! Try lower.".to_string();
                    }
                    Ordering::Less => {
                        self.message = "Too low! Try higher.".to_string();
                    }
                }
            }
            Ok(_) => {
                self.message = format!("Please enter a number between 1 and {}.", self.max_range);
            }
            Err(_) => {
                self.message = "Please enter a valid whole number.".to_string();
            }
        }
    }
}

impl Default for GuessApp {
    fn default() -> Self {
        let mut app = Self {
            secret: 0,
            input: String::new(),
            message: String::from("Welcome!"),
            attempts: 0,
            max_range: 100,
            won: false,
        };
        app.new_game();
        app
    }
}


impl eframe::App for GuessApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            egui::Frame::none()
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("🎯 Guess the Number");
                        ui.add_space(6.0);
                        ui.label("Pick the correct number...");

                        ui.add_space(8.0);


                        ui.horizontal(|ui| {
                            ui.label("Max:");
                            if ui.add(egui::Slider::new(&mut self.max_range, 10..=1000)).changed() {
                                self.new_game();
                            }
                            ui.label(format!("{}", self.max_range));
                        });

                        ui.add_space(6.0);


                        ui.horizontal(|ui| {
                            let text = ui.add(
                                egui::TextEdit::singleline(&mut self.input)
                                    .hint_text("Type a number and press Enter or Guess"),
                            );


                            if text.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                if !self.won {
                                   self.check_guess();
                                }
                            }

                            if ui.button("Guess").clicked() && !self.won {
                                self.check_guess();
                            }

                            if ui.button("New Game").clicked() {
                                self.new_game();
                            }
                        });

                        ui.add_space(8.0);

                        if self.won {
                            ui.colored_label(egui::Color32::from_rgb(30, 150, 60), &self.message);
                            ui.label(format!("Secret was: {}", self.secret));
                        } else {
                            ui.label(&self.message);
                        }

                        ui.add_space(6.0);
                        ui.label(format!("Attempts: {}", self.attempts));

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(6.0);
                        ui.small("Made with Rust + egui — press Enter to submit");
                    });
                });
        });


        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}

fn main() {
    let options = NativeOptions::default();

    eframe::run_native(
        "Guess the Number — Pretty",
        options,
        Box::new(|_cc| Box::new(GuessApp::default())),
    );
}
