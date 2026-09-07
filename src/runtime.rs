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
            top_screen: VirtualScreen::new(TOP_WIDTH, TOP_HEIGHT),
            bottom_screen: VirtualScreen::new(BOTTOM_WIDTH, BOTTOM_HEIGHT),
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
                Statement::Return => break,
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

                if let Some(Expression::Identifier(identifier)) = arguments.first() {
                    self.current_screen = match identifier.as_str() {
                        "GFX_BOTTOM" => ScreenTarget::Bottom,
                        _ => ScreenTarget::Top,
                    };
                }
            }

            "printf" => {
                if let Some(Expression::String(text)) = arguments.first() {
                    match self.current_screen {
                        ScreenTarget::Top => self.top_screen.write_text(text),
                        ScreenTarget::Bottom => self.bottom_screen.write_text(text),
                    }
                }
            }

            "gfxExit" => {
                self.initialized = false;
                self.console_initialized = false;
            }

            "gfxFlushBuffers" | "gfxSwapBuffers" => {}

            _ => {}
        }
    }
}