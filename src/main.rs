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
const GAP: f32 = 8.0;
const CANVAS_W: f32 = 400.0;
const CANVAS_H: f32 = TOP_H + GAP + BOTTOM_H;

fn main() -> eframe::Result {
    let project_path = std::env::args()
        .nth(1)
        .ok_or_else(|| {
            eframe::Error::AppCreation(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Usage: hbview3 <path-to-project>",
            )))
        })?;

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
            .with_inner_size([820.0, 1040.0])
            .with_min_inner_size([500.0, 620.0]),
        ..Default::default()
    };

    eframe::run_ui_native("HBView3", options, move |ui, _frame| {
        ui.ctx()
            .request_repaint_after(Duration::from_millis(100));

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

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show_inside(ui, |ui| {
                let available = ui.available_size();

                let logical_width = TOP_W;
                let logical_height = TOP_H + GAP + BOTTOM_H;

                let scale = (available.x / logical_width)
                    .min(available.y / logical_height)
                    .max(1.0);

                let display_width = logical_width * scale;
                let display_height = logical_height * scale;

                let origin = egui::pos2(
                    ui.max_rect().center().x - display_width * 0.5,
                    ui.max_rect().center().y - display_height * 0.5,
                );

                let top_rect = egui::Rect::from_min_size(
                    origin,
                    egui::vec2(
                        TOP_W * scale,
                        TOP_H * scale,
                    ),
                );

                let bottom_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        origin.x + (TOP_W - BOTTOM_W) * scale * 0.5,
                        origin.y + (TOP_H + GAP) * scale,
                    ),
                    egui::vec2(
                        BOTTOM_W * scale,
                        BOTTOM_H * scale,
                    ),
                );

                let painter = ui.painter();

                painter.rect_filled(
                    top_rect,
                    0.0,
                    egui::Color32::BLACK,
                );

                painter.rect_filled(
                    bottom_rect,
                    0.0,
                    egui::Color32::BLACK,
                );

                for (index, text) in runtime.text().iter().enumerate() {
                    if text.is_empty() {
                        continue;
                    }

                    let logical_position = egui::vec2(
                        8.0,
                        8.0 + index as f32 * 16.0,
                    );

                    let position = top_rect.min
                        + logical_position * scale;

                    painter.text(
                        position,
                        egui::Align2::LEFT_TOP,
                        text,
                        egui::FontId::monospace(13.0 * scale),
                        egui::Color32::WHITE,
                    );
                }
            });
    })
}