use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "WinFY (egui version)",
        options,
        Box::new(|_cc| Box::new(WinFY::default())),
    );
}

struct WinFY {
    input: String,
    output: String,
}

impl Default for WinFY {
    fn default() -> Self {
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

            if ui.button("Exit").clicked() {
                _frame.quit();
            }

            self.output = format!("'{}'", self.input);
        });
    }
}
