#[derive(Default)]
pub struct InputState {
    pub held: u32,
    pub down: u32,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_frame(&mut self) {
        self.down = 0;
    }

    pub fn press(&mut self, key: u32) {
        if self.held & key == 0 {
            self.down |= key;
        }

        self.held |= key;
    }

    pub fn release(&mut self, key: u32) {
        self.held &= !key;
    }

    pub fn keys_down(&self) -> u32 {
        self.down
    }

    pub fn keys_held(&self) -> u32 {
        self.held
    }
}

pub const KEY_A: u32 = 1 << 0;
pub const KEY_B: u32 = 1 << 1;
pub const KEY_X: u32 = 1 << 2;
pub const KEY_Y: u32 = 1 << 3;

pub const KEY_UP: u32 = 1 << 4;
pub const KEY_DOWN: u32 = 1 << 5;
pub const KEY_LEFT: u32 = 1 << 6;
pub const KEY_RIGHT: u32 = 1 << 7;

pub const KEY_START: u32 = 1 << 8;
pub const KEY_SELECT: u32 = 1 << 9;

pub const KEY_L: u32 = 1 << 10;
pub const KEY_R: u32 = 1 << 11;