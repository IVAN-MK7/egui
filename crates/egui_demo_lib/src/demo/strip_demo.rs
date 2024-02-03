use egui::Color32;
use egui_extras::{Size, StripBuilder};

/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default)]
pub struct StripDemo {}

impl super::Demo for StripDemo {
    fn name(&self) -> &'static str {
        "▣ Strip Demo"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for StripDemo {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.7))
                    .size(Size::remainder())
                    .horizontal(|mut strip| {
                        strip.strip(|builder| {
                            builder
                                .size(Size::exact(20.))
                                .size(Size::exact(20.))
                                .size(Size::exact(20.))
                                .size(Size::exact(20.))
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        ui.label("strip_label_1");
                                    });
                                    strip.cell(|ui| {
                                        ui.label("strip_label_2");
                                    });
                                    strip.cell(|ui| {
                                        ui.label("strip_label_3");
                                    });
                                    strip.cell(|ui| {
                                        ui.label("strip_label_4");
                                    });
                                });
                        });
                        strip.strip(|builder| {
                            builder
                                .size(Size::exact(20.))
                                .size(Size::exact(20.))
                                .size(Size::exact(20.))
                                .size(Size::exact(20.))
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("strip_label_1_1");
                                            ui.label("strip_label_1_2");
                                        });
                                    });
                                    strip.cell(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("strip_label_2_1");
                                            ui.label("strip_label_2_2");
                                        });
                                    });
                                    strip.cell(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("strip_label_3_1");
                                            ui.label("strip_label_3_2");
                                        });
                                    });
                                    strip.cell(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("strip_label_4_1");
                                        });
                                    });
                                });
                        });
                    });
            });
            ui.vertical_centered(
                |ui| if ui.button("centered_button_must_not_overlap").clicked() {},
            );
            ui.label("label_must_not_overlap_1");
            ui.label("label_must_not_overlap_2");
            ui.label("label_must_not_overlap_3");
            ui.label("label_must_not_overlap_4");
        });
    }
}
