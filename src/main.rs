mod parser;
mod project;

use project::Project;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;

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

    for source in &project.sources {
        let analysis = parser::parse_file(source)?;

        println!("{}", analysis.path);

        for function in &analysis.functions {
            println!("  function: {function}");
        }

        for call in &analysis.calls {
            println!("  call: {call}");
        }

        println!();
    }
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

    for source in &project.sources {
        println!("  [C/C++] {}", source.display());
    }

    println!();

    let sdl = sdl3::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;

    let window_width = TOP_W * SCALE;
    let window_height = (TOP_H + GAP + BOTTOM_H) * SCALE;

    let window = video
        .window("HBView3", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();

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

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        // Top screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas
            .fill_rect(Rect::new(0, 0, TOP_W * SCALE, TOP_H * SCALE))
            .map_err(|e| e.to_string())?;

        // Bottom screen
        canvas
            .fill_rect(Rect::new(
                ((TOP_W - BOTTOM_W) * SCALE / 2) as i32,
                ((TOP_H + GAP) * SCALE) as i32,
                BOTTOM_W * SCALE,
                BOTTOM_H * SCALE,
            ))
            .map_err(|e| e.to_string())?;

        canvas.present();
    }

    Ok(())
}
