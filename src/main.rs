//! Antarmuka GUI untuk program hitung weton (dina pitu + Pancawara).

use eframe::egui;
use weton_gui::{Weton, hitung_weton};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        // eframe 0.37 memakai wgpu secara default. Renderer glow (OpenGL) lebih
        // andal di mesin tanpa akselerasi GPU / session desktop tertentu.
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 300.0])
            .with_min_inner_size([340.0, 260.0])
            .with_title("Weton")
            .with_visible(true),
        ..Default::default()
    };

    eframe::run_native(
        "Weton",
        options,
        Box::new(|cc| {
            // Tema terang dengan teks gelap supaya tulisan mudah dibaca.
            let mut visuals = egui::Visuals::light();
            visuals.override_text_color = Some(egui::Color32::from_gray(15));
            visuals.weak_text_color = Some(egui::Color32::from_gray(80));
            visuals.weak_text_alpha = 1.0;
            cc.egui_ctx.set_visuals(visuals);
            Ok(Box::<WetonApp>::default())
        }),
    )
}

#[derive(Default)]
struct WetonApp {
    tanggal: String,
    bulan: String,
    tahun: String,
    /// Input terakhir yang sudah dihitung; dipakai untuk menghapus hasil lama
    /// saat pengguna mengubah salah satu kolom.
    input_terakhir: String,
    hasil: Option<Weton>,
    error: Option<String>,
}

impl WetonApp {
    fn input_saat_ini(&self) -> String {
        format!("{} {} {}", self.tanggal, self.bulan, self.tahun)
    }

    fn hitung(&mut self) {
        let input = self.input_saat_ini();
        match hitung_weton(&input) {
            Ok(weton) => {
                self.hasil = Some(weton);
                self.error = None;
            }
            Err(pesan) => {
                self.hasil = None;
                self.error = Some(pesan);
            }
        }
        self.input_terakhir = input;
    }

    fn reset(&mut self) {
        self.tanggal.clear();
        self.bulan.clear();
        self.tahun.clear();
        self.hasil = None;
        self.error = None;
        self.input_terakhir = self.input_saat_ini();
    }
}

impl eframe::App for WetonApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Hapus hasil/error lama begitu input berubah.
        if self.input_saat_ini() != self.input_terakhir {
            self.hasil = None;
            self.error = None;
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(
                egui::RichText::new("Weton")
                    .heading()
                    .strong()
                    .color(egui::Color32::from_gray(0)),
            );
            ui.label("Hitung hari dan pasaran dari tanggal lahir.");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Tanggal");
                ui.add(
                    egui::TextEdit::singleline(&mut self.tanggal)
                        .hint_text("dd")
                        .desired_width(60.0),
                );
                ui.label("Bulan");
                ui.add(
                    egui::TextEdit::singleline(&mut self.bulan)
                        .hint_text("mm")
                        .desired_width(60.0),
                );
                ui.label("Tahun");
                ui.add(
                    egui::TextEdit::singleline(&mut self.tahun)
                        .hint_text("yyyy")
                        .desired_width(70.0),
                );
            });

            ui.add_space(4.0);
            ui.label(egui::RichText::new("Contoh: 16 06 1990").small());

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                let enter_ditekan = ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button("Hitung").clicked() || enter_ditekan {
                    self.hitung();
                }
                if ui.button("Reset").clicked() {
                    self.reset();
                }
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            if let Some(pesan) = &self.error {
                ui.label(
                    egui::RichText::new(format!("Input tidak valid: {pesan}"))
                        .strong()
                        .color(egui::Color32::from_rgb(150, 0, 0)),
                );
            } else if let Some(weton) = &self.hasil {
                ui.label(
                    egui::RichText::new(weton.nama())
                        .heading()
                        .strong()
                        .color(egui::Color32::from_rgb(0, 90, 40)),
                );
                ui.horizontal(|ui| {
                    ui.label(format!("Hari: {}", weton.hari));
                    ui.label("   ");
                    ui.label(format!("Pasaran: {}", weton.pasaran));
                });
            } else {
                ui.label("Hasil akan muncul di sini.");
            }
        });
    }
}
