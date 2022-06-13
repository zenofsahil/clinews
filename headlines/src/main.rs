mod headlines;

use headlines::{ Headlines, NewsCardData };
use newsapi::NewsAPI;

fn fetch_news(api_key: &str) {
    if let Ok(response) = NewsAPI::new(api_key).fetch() {
        let articles = response.articles();
        for a in articles.iter() {
            let news = NewsCardData {
                title: a.title().to_string(),
                url: a.url().to_string(),
                description: a.description().to_string()
            };
        }
    }
}

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
