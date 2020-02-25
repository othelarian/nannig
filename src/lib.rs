use candelabre_core::CandlUpdate;
use glutin::event::{ModifiersState, VirtualKeyCode};

pub mod nannig_wins;
use nannig_wins::NannigGraphics;

// NannigMessage ==============================================================

pub enum NannigMessage {
    Nothing,
    Quit
}

// NannigState ================================================================

pub struct NannigState {
    redraw: bool
}

impl CandlUpdate<(), NannigGraphics> for NannigState {
    fn update(&mut self, _message: (), _graphics: &mut NannigGraphics) {}
}

impl NannigState {
    pub fn new() -> Self { Self {redraw: false} }

    pub fn need_redraw(&mut self) { if !self.redraw { self.redraw = true; } }

    pub fn redraw_asked(&mut self) -> bool {
        if self.redraw { self.redraw = false; true }
        else { false }
    }
}

// NannigStore ================================================================

pub struct NannigStore {
    ctrl_mod: bool,
    shift_mod: bool,
    alt_mod: bool,
    logo_mod: bool,
    fullscreen_mode: bool
}

impl NannigStore {
    pub fn new() -> Self {
        Self {
            ctrl_mod: false,
            shift_mod: false,
            alt_mod: false,
            logo_mod: false,
            fullscreen_mode: false
        }
    }

    pub fn handle_keycode(&mut self, keycode: VirtualKeyCode) -> NannigMessage {
        match keycode {
            /*
            VirtualKeyCode::C => {
                //
                // TODO : open configuration
                //
            }
            VirtualKeyCode::F => {
                //
                // TODO : go (NO TOGGLE) fullscreen and monitors
                //
            }
            VirtualKeyCode::S => {
                //
                // TODO : with mod keys, quit fullscreen mode
                //
            }
            */
            VirtualKeyCode::Q => {
                if !self.logo_mod && !self.fullscreen_mode &&
                    self.ctrl_mod && self.alt_mod {
                        NannigMessage::Quit
                } else { NannigMessage::Nothing }
            }
            /*
            VirtualKeyCode::Left => {
                //
                println!("left");
                //
                //
            }
            VirtualKeyCode::Right => {
                //
                println!("right");
                //
                //
            }
            VirtualKeyCode::Up => {
                //
                println!("up");
                //
            }
            VirtualKeyCode::Down => {
                //
                println!("down");
                //
            }
            */
            _ => {
                //
                // TODO : if in a middle of a sequence, stop it
                //
                NannigMessage::Nothing
            }
        }
    }

    pub fn update_mods(&mut self, mod_state: ModifiersState) {
        self.ctrl_mod = mod_state.ctrl();
        self.shift_mod = mod_state.shift();
        self.alt_mod = mod_state.alt();
        self.logo_mod = mod_state.logo();
    }

}
