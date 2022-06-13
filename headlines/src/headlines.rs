use serde::{ Serialize, Deserialize };
use eframe::App;
use eframe::egui::{
    Window,
    Color32,
    RichText,
    Layout,
    Vec2,
    Ui,
    Separator,
    TopBottomPanel,
    TextStyle,
    Button
};
use newsapi::NewsAPI;

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const RED: Color32 = Color32::from_rgb(255, 0, 0);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

fn fetch_news(api_key: &str, articles: &mut Vec<NewsCardData>) {
    if let Ok(response) = NewsAPI::new(api_key).fetch() {
        tracing::info!("Fetched!");
        let response_articles = response.articles();
        for a in response_articles.iter() {
            let news = NewsCardData {
                title: a.title().to_string(),
                url: a.url().to_string(),
                description: a.description().to_string()
            };
            articles.push(news);
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct HeadlinesConfig {
    dark_mode: bool,
    api_key: String
}

pub struct NewsCardData {
    pub title: String,
    pub url: String,
    pub description: String
}

#[derive(Default)]
pub struct Headlines {
    articles: Vec<NewsCardData>,
    config: HeadlinesConfig,
    api_key_initialized: bool
}

fn setup_custom_fonts(ctx: &eframe::egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = eframe::egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../MesloLGS_NF_Regular.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(eframe::egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(eframe::egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

impl Headlines {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        let config: HeadlinesConfig = confy::load("headlines").unwrap_or_default();

        let mut articles: Vec<NewsCardData> = Vec::new();

        if config.api_key.is_empty() {
            Headlines {
                api_key_initialized: false,
                articles,
                config
            }
        } else {
            fetch_news(&config.api_key, &mut articles);
            Headlines {
                api_key_initialized: true,
                articles,
                config
            }
        }
    }
  
    fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);

            let title = format!("> {}", a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }

            ui.add_space(PADDING);
            let description = RichText::new(&a.description).text_style(eframe::egui::TextStyle::Button);
            ui.label(description);
            
            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }

            ui.add_space(PADDING);
            ui.allocate_ui_with_layout( Vec2::new(ui.available_width(), 0.0), Layout::right_to_left(), |ui| {
                ui.hyperlink_to("read more...", &a.url);
            });
            ui.add_space(PADDING);
            ui.separator();
        }
    }

    fn render_top_panel(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            eframe::egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.label(RichText::new("((.))").text_style(TextStyle::Heading));
                });
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new(RichText::new("X").text_style(TextStyle::Body)));
                    if close_btn.clicked() {
                        frame.quit()
                    }
                    let refresh_btn = ui.add(Button::new(RichText::new("r").text_style(TextStyle::Body)));

                    let theme_btn = ui.add(Button::new(RichText::new("@").text_style(TextStyle::Body)));
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    fn render_config(&mut self, ctx: &eframe::egui::Context) {
        Window::new("Configuration").show(ctx, |ui| {
            ui.label("Enter your API_KEY for newsapi.org");
            let text_input = ui.text_edit_singleline(&mut self.config.api_key);
            if text_input.lost_focus() && ui.input().key_pressed(eframe::egui::Key::Enter) {
                if let Err(e) = confy::store("headlines", HeadlinesConfig {
                    dark_mode: self.config.dark_mode,
                    api_key: self.config.api_key.to_owned()
                }) {
                    tracing::error!("Failed saving the app state: {}", e);
                }

                self.api_key_initialized = true;

                tracing::info!("API key set");
            }
            ui.label("If you don't have an API key, create one at");
            ui.hyperlink("https://newsapi.org");
        });
    }
}

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if self.config.dark_mode {
            ctx.set_visuals(eframe::egui::Visuals::dark());
        } else {
            ctx.set_visuals(eframe::egui::Visuals::light());
        }

        if !self.api_key_initialized {
            self.render_config(ctx);
        } else {
            self.render_top_panel(ctx, frame);
            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                render_header(ui);
                eframe::egui::containers::ScrollArea::new([false, true])
                    .auto_shrink([false, false])
                    .always_show_scroll(false)
                    .show(ui, |ui| self.render_news_cards(ui));
                render_footer(ctx);
                });
        }
    }
}

fn render_footer(ctx: &eframe::egui::Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.label(RichText::new("API Source: newsapi.org").monospace());
            ui.hyperlink_to(
                RichText::new("zenofsahil/headlines").text_style(TextStyle::Monospace), 
                "https://github.com/emilk/egui"
            );
            ui.add_space(10.);
        });
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    ui.add(Separator::default().spacing(20.0));
}
