mod ir;
mod parser;
mod project;
mod runtime;

use project::Project;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use std::time::{Duration, SystemTime};

const TOP_W: u32 = 400;
const TOP_H: u32 = 240;
const BOTTOM_W: u32 = 320;
const BOTTOM_H: u32 = 240;
const GAP: u32 = 20;
const SCALE: u32 = 2;

fn main() -> Result<(), String> {
    let project_path = std::env::args()
        .nth(1)
        .ok_or_else(|| "Usage: hbview3 <path-to-project>".to_string())?;

    let project = Project::load(&project_path)?;

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

        let program = parser::parse_file(source)?;
        runtime.execute(&program);
    }

    for source in &project.sources {
        println!("  [C/C++] {}", source.display());
    }

    println!();

    let sdl = sdl3::init().map_err(|e| e.to_string())?;
    let ttf = sdl3::ttf::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;

    let window_width = TOP_W * SCALE;
    let window_height = (TOP_H + GAP + BOTTOM_H) * SCALE;

    let window = video
        .window("HBView3", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();

    let font_path = std::path::Path::new("Assets/Arial.ttf");

    let font = ttf.load_font(font_path, 16.0).map_err(|e| e.to_string())?;

    let mut events = sdl.event_pump().map_err(|e| e.to_string())?;
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

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
                let program = parser::parse_file(source)?;
                runtime.execute(&program);
            }

            for text in runtime.text() {
                println!("Reloaded text: {text}");
            }
        }

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas
            .fill_rect(Rect::new(0, 0, TOP_W * SCALE, TOP_H * SCALE))
            .map_err(|e| e.to_string())?;

        canvas
            .fill_rect(Rect::new(
                ((TOP_W - BOTTOM_W) * SCALE / 2) as i32,
                ((TOP_H + GAP) * SCALE) as i32,
                BOTTOM_W * SCALE,
                BOTTOM_H * SCALE,
            ))
            .map_err(|e| e.to_string())?;

        let text_color = Color::RGB(255, 255, 255);

    for (index, text) in runtime.text().iter().enumerate() {
        if text.is_empty() {
            continue;
        }

        let surface = font
            .render(text)
            .blended(text_color)
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let query = texture.query();

        canvas
            .copy(
                &texture,
                None,
                Rect::new(10, 10 + (index as i32 * 20), query.width, query.height),
            )
            .map_err(|e| e.to_string())?;
    }

        canvas.present();
    }

    Ok(())
}
