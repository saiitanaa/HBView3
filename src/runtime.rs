use crate::input::InputState;
use crate::ir::{Expression, Program, Statement};
use crate::screen::{
    VirtualScreen,
    TOP_HEIGHT,
    TOP_WIDTH,
    BOTTOM_HEIGHT,
    BOTTOM_WIDTH,
};

pub struct Runtime {
    initialized: bool,
    console_initialized: bool,
    current_screen: ScreenTarget,
    top_screen: VirtualScreen,
    bottom_screen: VirtualScreen,
    input: InputState,
}

enum ScreenTarget {
    Top,
    Bottom,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            initialized: false,
            console_initialized: false,
            current_screen: ScreenTarget::Top,
            top_screen: VirtualScreen::new(
                TOP_WIDTH,
                TOP_HEIGHT,
            ),
            bottom_screen: VirtualScreen::new(
                BOTTOM_WIDTH,
                BOTTOM_HEIGHT,
            ),
            input: InputState::new(),
        }
    }

    pub fn execute(&mut self, program: &Program) {
        let Some(main) = program
            .functions
            .iter()
            .find(|function| function.name == "main")
        else {
            return;
        };

        for statement in &main.body {
            match statement {
                Statement::Call {
                    name,
                    arguments,
                } => {
                    self.call(name, arguments);
                }

                Statement::Variable { name, .. } => {
                    println!("Variable: {name}");
                }

                Statement::Return => {
                    break;
                }
            }
        }
    }

    pub fn top_screen(&self) -> &VirtualScreen {
        &self.top_screen
    }

    pub fn bottom_screen(&self) -> &VirtualScreen {
        &self.bottom_screen
    }

    pub fn begin_frame(&mut self) {
        self.input.begin_frame();
    }

    pub fn press_key(&mut self, key: u32) {
        self.input.press(key);
    }

    pub fn release_key(&mut self, key: u32) {
        self.input.release(key);
    }

    pub fn keys_down(&self) -> u32 {
        self.input.keys_down()
    }

    pub fn keys_held(&self) -> u32 {
        self.input.keys_held()
    }

    fn call(
        &mut self,
        name: &str,
        arguments: &[Expression],
    ) {
        match name {
            "gfxInitDefault" => {
                self.initialized = true;
            }

            "consoleInit" => {
                self.console_initialized = true;

                if let Some(Expression::Identifier(identifier)) =
                    arguments.first()
                {
                    self.current_screen =
                        match identifier.as_str() {
                            "GFX_BOTTOM" => ScreenTarget::Bottom,
                            _ => ScreenTarget::Top,
                        };
                }
            }

            "printf" => {
                if let Some(Expression::String(text)) =
                    arguments.first()
                {
                    match self.current_screen {
                        ScreenTarget::Top => {
                            self.top_screen.write_text(text);
                        }

                        ScreenTarget::Bottom => {
                            self.bottom_screen.write_text(text);
                        }
                    }
                }
            }

            "gfxExit" => {
                self.initialized = false;
                self.console_initialized = false;
            }

            "gfxFlushBuffers" | "gfxSwapBuffers" => {}

            "hbvClear" => {
                let Some(Expression::Integer(color)) =
                    arguments.first()
                else {
                    return;
                };

                let color = *color as u32;

                match self.current_screen {
                    ScreenTarget::Top => {
                        self.top_screen.clear_color(color);
                    }

                    ScreenTarget::Bottom => {
                        self.bottom_screen.clear_color(color);
                    }
                }
            }

            "hbvDrawPixel" => {
                if arguments.len() < 3 {
                    return;
                }

                let (
                    Some(Expression::Integer(x)),
                    Some(Expression::Integer(y)),
                    Some(Expression::Integer(color)),
                ) = (
                    arguments.first(),
                    arguments.get(1),
                    arguments.get(2),
                )
                else {
                    return;
                };

                let x = (*x).max(0) as usize;
                let y = (*y).max(0) as usize;
                let color = *color as u32;

                match self.current_screen {
                    ScreenTarget::Top => {
                        self.top_screen.set_pixel(
                            x,
                            y,
                            color,
                        );
                    }

                    ScreenTarget::Bottom => {
                        self.bottom_screen.set_pixel(
                            x,
                            y,
                            color,
                        );
                    }
                }
            }

            "hbvDrawRect" => {
                if arguments.len() < 5 {
                    return;
                }

                let (
                    Some(Expression::Integer(x)),
                    Some(Expression::Integer(y)),
                    Some(Expression::Integer(width)),
                    Some(Expression::Integer(height)),
                    Some(Expression::Integer(color)),
                ) = (
                    arguments.first(),
                    arguments.get(1),
                    arguments.get(2),
                    arguments.get(3),
                    arguments.get(4),
                )
                else {
                    return;
                };

                let x = (*x).max(0) as usize;
                let y = (*y).max(0) as usize;
                let width = (*width).max(0) as usize;
                let height = (*height).max(0) as usize;
                let color = *color as u32;

                match self.current_screen {
                    ScreenTarget::Top => {
                        self.top_screen.draw_rect(
                            x,
                            y,
                            width,
                            height,
                            color,
                        );
                    }

                    ScreenTarget::Bottom => {
                        self.bottom_screen.draw_rect(
                            x,
                            y,
                            width,
                            height,
                            color,
                        );
                    }
                }
            }

            "hidScanInput" => {
                self.input.begin_frame();
            }

            "hidKeysDown" => {
                let _ = self.input.keys_down();
            }

            "hidKeysHeld" => {
                let _ = self.input.keys_held();
            }

            _ => {}
        }
    }
}