use std::path::Path;
use eframe::egui;
use crate::loader::load_ppm;

pub struct App {
    pub path: String,
    texture: Option<egui::TextureHandle>,
    image_size: Option<egui::Vec2>,
    error_message: Option<String>,
}

impl App {
    pub fn new(path: String) -> Self {
        App {
            path,
            texture: None,
            image_size: None,
            error_message: None,
        }
    }
    fn load_image(&mut self, ctx: &egui::Context) {
        match load_ppm(&self.path) {
            Ok((color_image, size)) => {
                let tex =
                    ctx.load_texture("ppm-Image", color_image, egui::TextureOptions::default());

                self.texture = Some(tex);
                self.image_size = Some(egui::Vec2::new(size.0 as f32, size.1 as f32));
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(e);
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.texture.is_none() && self.error_message.is_none() {
            self.load_image(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_rect = ui.available_rect_before_wrap();
            let bottom_panel_height = 25.0;
            let image_rect = egui::Rect::from_min_size(
                available_rect.min,
                egui::Vec2::new(
                    available_rect.width(),
                    available_rect.height() - bottom_panel_height,
                ),
            );

            ui.scope_builder(egui::UiBuilder::new().max_rect(image_rect), |ui| {
                if let Some(texture) = &self.texture {
                    if let Some(image_size) = self.image_size {
                        let available_size = ui.available_size();
                        let scale_x = available_size.x / image_size.x;
                        let scale_y = available_size.y / image_size.y;
                        let scale = scale_x.min(scale_y).min(1.0);

                        let display_size = image_size * scale;
                        ui.allocate_ui_with_layout(
                            available_size,
                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                ui.image((texture.id(), display_size));
                            },
                        );
                    }
                } else if let Some(error_msg) = &self.error_message {
                    ui.centered_and_justified(|ui| {
                        ui.label(error_msg);
                    });
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("Loading image...");
                    });
                }
            });

            let bottom_rect = egui::Rect::from_min_size(
                egui::Pos2::new(available_rect.min.x, available_rect.min.y + image_rect.height()),
                egui::Vec2::new(available_rect.width(), bottom_panel_height),
            );

            ui.scope_builder(egui::UiBuilder::new().max_rect(bottom_rect), |ui| {
                ui.separator();

                if let Some(image_size) = self.image_size {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "File: {}",
                            Path::new(&self.path)
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                        ));
                        ui.separator();
                        ui.label(format!("Size: {:.0} × {:.0}", image_size.x, image_size.y));
                    });
                } else {
                    ui.label(format!("File: {}", self.path));
                }
            });
        });
    }
}