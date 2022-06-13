mod headlines;

use headlines::Headlines;

fn main() {
    tracing_subscriber::fmt::init();
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(eframe::egui::Vec2::new(540., 960.));
    eframe::run_native(
        "Headlines",
        options,
        Box::new(|cc| Box::new(Headlines::new(cc))),
    );
}
