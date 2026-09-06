use crate::ir::{Program, Statement};

pub struct Runtime {
    initialized: bool,
}

impl Runtime {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn execute(&mut self, program: &Program) {
        let Some(main) = program
            .functions
            .iter()
            .find(|function| function.name == "main")
        else {
            println!("runtime: main not found");
            return;
        };

        for statement in &main.body {
            match statement {
                Statement::Call(name) => self.call(name.as_str()),
                Statement::Return => break,
            }
        }
    }

    fn call(&mut self, name: &str) {
        match name {
            "gfxInitDefault" => self.gfx_init_default(),
            "gfxExit" => self.gfx_exit(),
            "gfxFlushBuffers" => self.gfx_flush_buffers(),
            "gfxSwapBuffers" => self.gfx_swap_buffers(),
            "hidScanInput" => self.hid_scan_input(),
            _ => {}
        }
    }

    fn gfx_init_default(&mut self) {
        self.initialized = true;
        println!("runtime: gfxInitDefault");
    }

    fn gfx_exit(&mut self) {
        self.initialized = false;
        println!("runtime: gfxExit");
    }

    fn gfx_flush_buffers(&self) {
        println!("runtime: gfxFlushBuffers");
    }

    fn gfx_swap_buffers(&self) {
        println!("runtime: gfxSwapBuffers");
    }

    fn hid_scan_input(&self) {
        println!("runtime: hidScanInput");
    }
}
