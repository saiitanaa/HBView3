use crate::ir::{Expression, Program, Statement};
use crate::screen::VirtualScreen;

pub struct Runtime {
    initialized: bool,
    console_initialized: bool,
    current_screen: ScreenTarget,
    top_screen: VirtualScreen,
    bottom_screen: VirtualScreen,
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
            top_screen: VirtualScreen::new(400, 240),
            bottom_screen: VirtualScreen::new(320, 240),
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
                Statement::Call { name, arguments } => {
                    self.call(name, arguments);
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

    fn call(&mut self, name: &str, arguments: &[Expression]) {
        match name {
            "gfxInitDefault" => {
                self.initialized = true;
            }

            "consoleInit" => {
                self.console_initialized = true;

                if let Some(Expression::Identifier { name: identifier }) = arguments.first() {
                    self.current_screen = match identifier.as_str() {
                        "GFX_BOTTOM" => ScreenTarget::Bottom,
                        _ => ScreenTarget::Top,
                    };
                }
            }

            "printf" => {
                if let Some(Expression::String { value: text }) = arguments.first() {
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

            "hbvClear" => {
                if let Some(color) = Self::integer_argument(arguments, 0) {
                    self.current_screen_mut().clear(color as u32);
                }
            }

            "hbvDrawPixel" => {
                let Some(x) = Self::integer_argument(arguments, 0) else {
                    return;
                };

                let Some(y) = Self::integer_argument(arguments, 1) else {
                    return;
                };

                let Some(color) = Self::integer_argument(arguments, 2) else {
                    return;
                };

                self.current_screen_mut()
                    .draw_pixel(x, y, color as u32);
            }

            "hbvDrawRect" => {
                let Some(x) = Self::integer_argument(arguments, 0) else {
                    return;
                };

                let Some(y) = Self::integer_argument(arguments, 1) else {
                    return;
                };

                let Some(width) = Self::integer_argument(arguments, 2) else {
                    return;
                };

                let Some(height) = Self::integer_argument(arguments, 3) else {
                    return;
                };

                let Some(color) = Self::integer_argument(arguments, 4) else {
                    return;
                };

                self.current_screen_mut()
                    .draw_rect(x, y, width, height, color as u32); 
            }
            _ => {}
        }
    }

    fn current_screen_mut(&mut self) -> &mut VirtualScreen {
        match self.current_screen {
            ScreenTarget::Top => &mut self.top_screen,
            ScreenTarget::Bottom => &mut self.bottom_screen,
        }
    }
    
    fn integer_argument(arguments: &[Expression], index: usize) -> Option<usize> {
        match arguments.get(index) {
            Some(Expression::Integer { value }) => (*value).try_into().ok(),
            _ => None,
        }
    }

}