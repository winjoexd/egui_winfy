use eframe::egui;
mod fy_lib;
use fy_lib::fy_handle;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "WinFY (egui version)",
        options,
        Box::new(|_cc| Box::new(WinFY::new(_cc))),
    );
}

struct WinFY {
    input: String,
    output: String,
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/cjkfonts_allseto/cjkFonts_allseto_v1.11.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

impl WinFY {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            input: "".to_owned(),
            output: "".to_owned(),
        }
    }
}

impl eframe::App for WinFY {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("WinFY (egui version)");
            ui.horizontal(|ui| {
                ui.text_edit_multiline(&mut self.input);
                ui.text_edit_multiline(&mut self.output);
            });

            ui.horizontal(|ui| {
                if ui.button("WinFY").clicked() {
                    self.output = fy_handle(self.input.to_string(), "ENG".to_string(), "CHT".to_string());
                }
                
                if ui.button("Exit").clicked() {
                    _frame.quit();
                }
            });
        });
    }
}
