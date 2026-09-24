use crate::ir::{Expression, Program, Statement};
use crate::screen::VirtualScreen;
use std::collections::HashMap;

pub struct Runtime {
    initialized: bool,
    console_initialized: bool,
    current_screen: ScreenTarget,
    top_screen: VirtualScreen,
    bottom_screen: VirtualScreen,
    variables: HashMap<String, Expression>,
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
            variables: HashMap::new(),
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

                Statement::Variable { name, value } => {
                    self.variables.insert(name.clone(), value.clone());
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
                let Some(Expression::String { value: format }) = arguments.first() else {
                    return;
                };

                let mut output = String::new();
                let mut argument_index = 1;
                let mut chars = format.chars();

                while let Some(character) = chars.next() {
                    if character != '%' {
                        output.push(character);
                        continue;
                    }

                    let Some(format_character) = chars.next() else {
                        output.push('%');
                        break;
                    };

                    match format_character {
                        '%' => {
                            output.push('%');
                        }

                        'd' => {
                            if let Some(argument) = arguments.get(argument_index) {
                                match argument {
                                    Expression::Integer { value } => {
                                        output.push_str(&value.to_string());
                                    }

                                    Expression::Identifier { name } => {
                                        if let Some(Expression::Integer { value }) =
                                            self.variables.get(name)
                                        {
                                            output.push_str(
                                                &value.to_string(),
                                            );
                                        }
                                    }

                                    _ => {}
                                }
                            }

                            argument_index += 1;
                        }

                        _ => {
                            output.push('%');
                            output.push(format_character);
                        }
                    }
                }

                match self.current_screen {
                    ScreenTarget::Top => {
                        self.top_screen.write_text(&output);
                    }

                    ScreenTarget::Bottom => {
                        self.bottom_screen.write_text(&output);
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

            "hbvFillRect" => {
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

                self.current_screen_mut().fill_rect(x, y, width, height, color as u32);
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

                self.current_screen_mut().draw_pixel(x, y, color as u32);
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

                self.current_screen_mut().draw_rect(x, y, width, height, color as u32); 
            }

            "hbvDrawLine" => {
                let Some(x1) = Self::integer_argument(arguments, 0) else {
                    return;
                };

                let Some(y1) = Self::integer_argument(arguments, 1) else {
                    return;
                };

                let Some(x2) = Self::integer_argument(arguments, 2) else {
                    return;
                };

                let Some(y2) = Self::integer_argument(arguments, 3) else {
                    return;
                };

                let Some(color) = Self::integer_argument(arguments, 4) else {
                    return;
                };

                self.current_screen_mut().draw_line(x1, y1, x2, y2, color as u32);
            }

            "hbvDrawCircle" => {
                let Some(center_x) = Self::integer_argument(arguments, 0) else {
                    return;
                };

                let Some(center_y) = Self::integer_argument(arguments, 1) else {
                    return;
                };

                let Some(radius) = Self::integer_argument(arguments, 2) else {
                    return;
                };

                let Some(color) = Self::integer_argument(arguments, 3) else {
                    return;
                };

                self.current_screen_mut().draw_circle(center_x, center_y, radius, color as u32);
            }

            "hbvDrawEllipse" => {
                let Some(center_x) = Self::integer_argument(arguments, 0) else {
                    return;
                };

                let Some(center_y) = Self::integer_argument(arguments, 1) else {
                    return;
                };

                let Some(radius_x) = Self::integer_argument(arguments, 2) else {
                    return;
                };

                let Some(radius_y) = Self::integer_argument(arguments, 3) else {
                    return;
                };

                let Some(color) = Self::integer_argument(arguments, 4) else {
                    return;
                };

                self.current_screen_mut().draw_ellipse(center_x, center_y, radius_x, radius_y, color as u32);
            }

            "hbvDrawTriangle" => {
                let Some(x1) = Self::integer_argument(arguments, 0) else {
                    return;
                };

                let Some(y1) = Self::integer_argument(arguments, 1) else {
                    return;
                };

                let Some(x2) = Self::integer_argument(arguments, 2) else {
                    return;
                };
                
                let Some(y2) = Self::integer_argument(arguments, 3) else {
                    return;
                };

                let Some(x3) = Self::integer_argument(arguments, 4) else {
                    return;
                };

                let Some(y3) = Self::integer_argument(arguments, 5) else {
                    return;
                };

                let Some(color) = Self::integer_argument(arguments, 6) else {
                    return;
                };

                self.current_screen_mut()
                    .draw_triangle(
                        x1,
                        y1,
                        x2,
                        y2,
                        x3,
                        y3,
                        color as u32,
                    );
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