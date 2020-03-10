use candelabre_core::CandlUpdate;
use glutin::event::{ModifiersState, VirtualKeyCode};
use glutin::window::WindowId;

pub mod nannig_wins;
use nannig_wins::NannigWinType;

// NannigMessage ==============================================================

pub enum NannigMessage {
    Classic,
    ConfigClose,
    ConfigOpen,
    Fullscreen,
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
    konami_phase: u16,
    classic_win: Option<WindowId>,
    config_win: Option<WindowId>,
    fullscreen_wins: Vec<WindowId>
}

impl NannigStore {
    pub fn new() -> Self {
        Self {
            ctrl_mod: false,
            shift_mod: false,
            alt_mod: false,
            logo_mod: false,
            fullscreen_mode: false,
            konami_phase: 0,
            classic_win: None,
            config_win: None,
            fullscreen_wins: Vec::new()
        }
    }

    pub fn handle_keycode(&mut self, keycode: VirtualKeyCode) -> NannigMessage {
        match keycode {
            VirtualKeyCode::A => {
                if self.fullscreen_mode {
                    if self.konami_phase == 8 { self.konami_phase = 9; }
                    else { self.konami_phase = 0; }
                }
                NannigMessage::Nothing
            }
            VirtualKeyCode::B => {
                if self.fullscreen_mode {
                    let valid = self.konami_phase == 9;
                    self.konami_phase = 0;
                    if valid {
                        self.fullscreen_mode = false;
                        NannigMessage::Classic
                    }
                    else { NannigMessage::Nothing }
                }
                else { NannigMessage::Nothing }
            }
            VirtualKeyCode::F => {
                if !self.fullscreen_mode && !self.logo_mod &&
                    self.ctrl_mod && self.alt_mod
                {
                    self.fullscreen_mode = true;
                    self.konami_phase = 0;
                    NannigMessage::Fullscreen
                }
                else {
                    if self.fullscreen_mode { self.konami_phase = 0; }
                    NannigMessage::Nothing
                }
            }
            VirtualKeyCode::O => {
                if !self.logo_mod && !self.fullscreen_mode &&
                    self.ctrl_mod && self.alt_mod
                {
                    if self.config_win.is_none() {
                        NannigMessage::ConfigOpen
                    } else {
                        NannigMessage::ConfigClose
                    }
                } else {
                    if self.fullscreen_mode { self.konami_phase = 0; }
                    NannigMessage::Nothing
                }
            }
            VirtualKeyCode::Q => {
                if !self.logo_mod && !self.fullscreen_mode &&
                    self.ctrl_mod && self.alt_mod {
                        NannigMessage::Quit
                } else {
                    if self.fullscreen_mode { self.konami_phase = 0; }
                    NannigMessage::Nothing
                }
            }
            VirtualKeyCode::Left => {
                if self.fullscreen_mode {
                    if self.konami_phase == 4 || self.konami_phase == 6 {
                        self.konami_phase += 1;
                    } else { self.konami_phase = 0; }
                }
                NannigMessage::Nothing
            }
            VirtualKeyCode::Right => {
                if self.fullscreen_mode {
                    if self.konami_phase == 5 || self.konami_phase == 7 {
                        self.konami_phase += 1;
                    } else { self.konami_phase = 0; }
                }
                NannigMessage::Nothing
            }
            VirtualKeyCode::Up => {
                if self.fullscreen_mode {
                    if self.konami_phase < 2 { self.konami_phase += 1; }
                    else { self.konami_phase = 0; }
                }
                NannigMessage::Nothing
            }
            VirtualKeyCode::Down => {
                if self.fullscreen_mode {
                    if self.konami_phase == 2 || self.konami_phase == 3 {
                        self.konami_phase += 1;
                    } else { self.konami_phase = 0; }
                }
                NannigMessage::Nothing
            }
            _ => {
                if self.fullscreen_mode { self.konami_phase = 0; }
                NannigMessage::Nothing
            }
        }
    }

    pub fn get_classic_win(&self) -> Option<WindowId> {
        self.classic_win.clone()
    }

    pub fn set_classic_win(&mut self, classic_win: Option<WindowId>) {
        self.classic_win = classic_win;
    }

    pub fn get_config_win(&self) -> Option<WindowId> {
        self.config_win.clone()
    }

    pub fn set_config_win(&mut self, config_win: Option<WindowId>) {
        self.config_win = config_win;
    }

    pub fn get_fullscreen_wins(&self) -> &Vec<WindowId> { &self.fullscreen_wins }

    pub fn add_fullscreen_win(&mut self, win_id: WindowId) {
        self.fullscreen_wins.push(win_id);
    }

    pub fn clear_fullscreen_wins(&mut self) {
        self.fullscreen_wins.clear();
    }

    pub fn update_mods(&mut self, mod_state: ModifiersState) {
        self.ctrl_mod = mod_state.ctrl();
        self.shift_mod = mod_state.shift();
        self.alt_mod = mod_state.alt();
        self.logo_mod = mod_state.logo();
    }

}
