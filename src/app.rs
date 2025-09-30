use eframe::egui;

pub struct App {
    pub path: String,
}

impl App {
    pub fn new(path: String) -> Self {
        App { path }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PPM Viewer");
            ui.separator();
            ui.label(format!("Path: {}", self.path));
        });
    }
}