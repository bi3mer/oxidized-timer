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
            time: 0
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_secs(1));

        egui::CentralPanel::default().show(ctx, |ui| {
            // handle timer if active
            match (self.current, self.duration) {
                (Some(current), Some(duration)) => {
                    if current.elapsed() > duration {
                        beep::play_beep(0.8);
                        self.reset();
                    } else {
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
                ui.add(egui::Slider::new(&mut self.time, 0..=60).text("Time"));
                if ui.button("Start").clicked() {
                    self.current = Some(Instant::now());
                    self.duration = Some(Duration::from_secs(self.time * 60))
                }
            }
        });
    }
}