use nvg::Color;
use std::collections::HashMap;

// NannigCfg ==================================================================

pub struct NannigCfg {
    //
    profiles: HashMap<String, NannigCfgProfile>,
    //
    second: NannigCfgGroup
    //
}

impl NannigCfg {
    pub fn check_conf_file() {
        //
        //
    }

    pub fn generate_default_file() {
        //
        //
    }
}

// NannigCfg Components =======================================================

pub struct NannigCfgArc {
    //
    //
}

pub struct NannigCfgGroup {
    arc: Option<NannigCfgArc>,
    needle: Option<NannigCfgNeedle>
    //
}



impl NannigCfgGroup {
    pub fn new() -> Self {
        Self {
            arc: None,
            needle: None
        }
    }
}



pub struct NannigCfgNeedle {
    //
    color: Color,
    curr_rot: f32,
    length: u32,
    negative: u32, //if the needle start before its rotational point
    thickness: u32
    //
}

pub struct NannigCfgProfile {
    //
    // TODO : this is a string which served as a format to share and save config
    //
}

impl NannigCfgProfile {
    pub fn default() -> Self {
        //
        Self {}
        //
    }
}
