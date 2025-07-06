use eframe::egui;
use eframe::egui::ViewportBuilder;
use std::process::{Command, Stdio};
use std::io;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 180.0])
            .with_title("Copiar página PDF"),
        ..Default::default()
    };
    eframe::run_native("Copiar página PDF", options, Box::new(|_cc| Box::<MyApp>::default()))
}

#[derive(Default)]
struct MyApp {
    pdf_path: Option<String>,
    page_number: String,
    output_msg: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Copiar uma página de PDF como imagem:");

            if ui.button("Selecionar PDF").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("PDF", &["pdf"])
                    .pick_file()
                {
                    self.pdf_path = Some(path.display().to_string());
                }
            }

            if let Some(ref path) = self.pdf_path {
                ui.label(format!("Selecionado: {}", path));
            }

            ui.horizontal(|ui| {
                ui.label("Página:");
                ui.text_edit_singleline(&mut self.page_number);
            });

            if ui.button("Executar").clicked() {
                match (self.pdf_path.clone(), self.page_number.parse::<u32>()) {
                    (Some(path), Ok(page)) => {
                        let pdftoppm = Command::new("pdftoppm")
                            .args(&["-png", "-f", &page.to_string(), "-l", &page.to_string(), &path])
                            .stdout(Stdio::piped())
                            .spawn();

                        match pdftoppm {
                            Ok(mut child) => {
                                let wl_copy = Command::new("wl-copy")
                                    .stdin(Stdio::piped())
                                    .spawn();

                                match wl_copy {
                                    Ok(mut wl_child) => {
                                        if let Some(ref mut ppm_out) = child.stdout {
                                            if let Some(ref mut wl_in) = wl_child.stdin {
                                                io::copy(ppm_out, wl_in).unwrap_or(0);
                                            }
                                        }
                                        let _ = child.wait();
                                        let _ = wl_child.wait();
                                        self.output_msg = "Página copiada para a área de transferência!".to_string();
                                    }
                                    Err(e) => {
                                        self.output_msg = format!("Erro ao iniciar wl-copy: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                self.output_msg = format!("Erro ao iniciar pdftoppm: {}", e);
                            }
                        }
                    }
                    _ => self.output_msg = "Caminho ou número da página inválido!".to_string(),
                }
            }

            if !self.output_msg.is_empty() {
                ui.separator();
                ui.label(&self.output_msg);
            }
        });
    }
}
