use std::time::{Instant, Duration};
use eframe::egui;
use egui::Vec2;

mod beep;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(300.0, 100.0));
    eframe::run_native(
        "Oxidized Timer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    current: Option<Instant>,
    duration: Option<Duration>,
    time: u64
}

impl MyApp {
    fn reset(&mut self) {
        self.current = None;
        self.duration = None;
        self.time = 2;
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current: None,
            duration: None,
            time: 2
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            // handle timer if active
            match (self.current, self.duration) {
                (Some(current), Some(duration)) => {
                    if current.elapsed() > duration {
                        beep::play_beep(0.8);
                        self.reset();
                    } else {
                        ctx.request_repaint_after(Duration::from_secs(1));
                        self.time = (duration - current.elapsed()).as_secs();
                    }
                },
                (_, _) => {},
            }
            
            // draw UI, which changes based on whether the timer is active
            if self.current.is_some() {
                ui.heading(format!("Time left: {}", self.time));
                if ui.button("Cancel").clicked() {
                    self.reset();
                }
            } else {
                ui.vertical(|ui| {
                    ui.add(egui::Slider::new(&mut self.time, 0..=60).text("Time"));
                    ui.add_space(3.0);
                    if ui.button("Start").clicked() {
                        self.current = Some(Instant::now());
                        self.duration = Some(Duration::from_secs(self.time * 60))
                    }

                    // time buttons...
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("2").clicked() {
                            self.current = Some(Instant::now());
                            self.duration = Some(Duration::from_secs(2 * 60));
                        }

                        if ui.button("5").clicked() {
                            self.current = Some(Instant::now());
                            self.duration = Some(Duration::from_secs(5 * 60));
                        }

                        if ui.button("10").clicked() {
                            self.current = Some(Instant::now());
                            self.duration = Some(Duration::from_secs(10 * 60));
                        }

                        if ui.button("20").clicked() {
                            self.current = Some(Instant::now());
                            self.duration = Some(Duration::from_secs(20 * 60));
                        }
                    });
                });
                
            }
        });
    }
}