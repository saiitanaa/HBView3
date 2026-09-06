mod ir;
mod parser;
mod project;
mod runtime;

use eframe::egui;
use project::Project;
use std::time::{Duration, SystemTime};

const TOP_W: f32 = 400.0;
const TOP_H: f32 = 240.0;
const BOTTOM_W: f32 = 320.0;
const BOTTOM_H: f32 = 240.0;
const GAP: f32 = 20.0;
const SCALE: f32 = 2.0;

fn main() -> eframe::Result {
    let project_path = std::env::args()
        .nth(1)
        .ok_or_else(|| eframe::Error::AppCreation(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Usage: hbview3 <path-to-project>",
        ))))?;

    let project = Project::load(&project_path)
        .map_err(|error| eframe::Error::AppCreation(Box::new(std::io::Error::other(error))))?;

    println!("================================");
    println!("|           HBView3            |");
    println!("================================");
    println!();
    println!("Project : {}", project.name());
    println!("Root    : {}", project.root.display());
    println!();
    println!("Sources : {}", project.sources.len());
    println!("Headers : {}", project.headers.len());
    println!("Assets  : {}", project.assets.len());
    println!(
        "Makefile: {}",
        if project.makefile.is_some() {
            "yes"
        } else {
            "no"
        }
    );
    println!();

    let mut runtime = runtime::Runtime::new();
    let mut modified_times = Vec::new();

    for source in &project.sources {
        let modified = std::fs::metadata(source)
            .and_then(|metadata| metadata.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        modified_times.push(modified);

        let program = parser::parse_file(source)
            .map_err(|error| eframe::Error::AppCreation(Box::new(std::io::Error::other(error))))?;

        runtime.execute(&program);
    }

    for source in &project.sources {
        println!("  [C/C++] {}", source.display());
    }

    println!();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("HBView3")
            .with_inner_size([
                TOP_W * SCALE + 80.0,
                (TOP_H + GAP + BOTTOM_H) * SCALE + 80.0,
            ]),
        ..Default::default()
    };

    eframe::run_ui_native("HBView3", options, move |ui, _frame| {
        let mut changed = false;

        for (index, source) in project.sources.iter().enumerate() {
            let modified = std::fs::metadata(source)
                .and_then(|metadata| metadata.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH);

            if modified != modified_times[index] {
                modified_times[index] = modified;
                changed = true;
            }
        }

        if changed {
            println!("Source changed, reloading...");

            runtime = runtime::Runtime::new();

            for source in &project.sources {
                match parser::parse_file(source) {
                    Ok(program) => runtime.execute(&program),
                    Err(error) => {
                        println!("Failed to reload {}: {error}", source.display());
                    }
                }
            }

            for text in runtime.text() {
                println!("Reloaded text: {text}");
            }
        }

        ui.ctx()
            .request_repaint_after(Duration::from_millis(100));

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal_centered(|ui| {
                let screen_area = egui::vec2(TOP_W * SCALE, TOP_H * SCALE);

                let (top_rect, _) = ui.allocate_exact_size(
                    screen_area,
                    egui::Sense::hover(),
                );

                let painter = ui.painter();

                painter.rect_filled(
                    top_rect,
                    0.0,
                    egui::Color32::BLACK,
                );

                for (index, text) in runtime.text().iter().enumerate() {
                    if text.is_empty() {
                        continue;
                    }

                    let position = top_rect.min
                        + egui::vec2(
                            10.0,
                            10.0 + index as f32 * 20.0,
                        );

                    painter.text(
                        position,
                        egui::Align2::LEFT_TOP,
                        text,
                        egui::FontId::monospace(16.0),
                        egui::Color32::WHITE,
                    );
                }
            });

            ui.add_space(GAP * SCALE);

            ui.horizontal_centered(|ui| {
                let screen_area = egui::vec2(
                    BOTTOM_W * SCALE,
                    BOTTOM_H * SCALE,
                );

                let (bottom_rect, _) = ui.allocate_exact_size(
                    screen_area,
                    egui::Sense::click_and_drag(),
                );

                ui.painter().rect_filled(
                    bottom_rect,
                    0.0,
                    egui::Color32::BLACK,
                );
            });
        });
    })
}