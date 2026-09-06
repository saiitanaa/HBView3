use crate::ir::{Expression, Program, Statement};

pub struct Runtime {
    initialized: bool,
    console_initialized: bool,
    text: Vec<String>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            initialized: false,
            console_initialized: false,
            text: Vec::new(),
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

    pub fn text(&self) -> &[String] {
        &self.text
    }

    fn call(&mut self, name: &str, arguments: &[Expression]) {
        match name {
            "gfxInitDefault" => {
                self.initialized = true;
            }

            "consoleInit" => {
                self.console_initialized = true;
            }

            "printf" => {
                if let Some(Expression::String(text)) = arguments.first() {
                    self.text.push(text.clone());
                }
            }

            "gfxExit" => {
                self.initialized = false;
            }

            "gfxFlushBuffers" | "gfxSwapBuffers" => {}

            _ => {}
        }
    }
}
