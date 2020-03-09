use candelabre_core::CandlUpdate;
use glutin::event::{ModifiersState, VirtualKeyCode};

pub mod nannig_wins;
use nannig_wins::NannigWinType;

// NannigMessage ==============================================================

pub enum NannigMessage {
    Config,
    Nothing,
    Quit
}

// NannigState ================================================================

pub struct NannigState {
    redraw: bool,
    //
    win_type: NannigWinType
    //
}

impl CandlUpdate<()> for NannigState {
    fn update(&mut self, _: ()) {}
}

impl NannigState {
    pub fn new(win_type: NannigWinType) -> Self {
        Self {
            redraw: false,
            //
            win_type
            //
        }
    }

    pub fn get_type(&self) -> NannigWinType { self.win_type.clone() }

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
    fullscreen_mode: bool,
    config_open: bool
}

impl NannigStore {
    pub fn new() -> Self {
        Self {
            ctrl_mod: false,
            shift_mod: false,
            alt_mod: false,
            logo_mod: false,
            fullscreen_mode: false,
            config_open: false
        }
    }

    pub fn handle_keycode(&mut self, keycode: VirtualKeyCode) -> NannigMessage {
        match keycode {
            VirtualKeyCode::O => {
                if !self.logo_mod && !self.fullscreen_mode &&
                    !self.ctrl_mod && self.alt_mod {
                        self.config_open = true;
                        NannigMessage::Config
                } else { NannigMessage::Nothing }
            }
            /*
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

    pub fn toggle_config(&mut self) { self.config_open = !self.config_open; }

    pub fn update_mods(&mut self, mod_state: ModifiersState) {
        self.ctrl_mod = mod_state.ctrl();
        self.shift_mod = mod_state.shift();
        self.alt_mod = mod_state.alt();
        self.logo_mod = mod_state.logo();
    }

}
