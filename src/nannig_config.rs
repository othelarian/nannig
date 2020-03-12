//use nvg::Color;
use std::collections::HashMap;

// NannigCfgManager ===========================================================

pub struct NannigCfgManager {
    //
    _profiles: HashMap<String, NannigCfgProfile>,
    //
    _select: String
    //
}

impl NannigCfgManager {
    pub fn check_conf_file() -> bool {
        //
        //
        false
        //
    }

    pub fn generate_config(&self, _select: Option<String>)
    -> Result<NannigCfg, &'static str> {
        //
        //
        Err("")
        //
    }

    pub fn generate_default_file() -> Self {
        //
        //
        NannigCfgManager::new()
        //
    }

    fn new() -> Self {
        //
        Self {
            _profiles: HashMap::default(),
            _select: String::from("aaaa")
        }
        //
    }

    pub fn read_conf_file() -> Self {
        //
        NannigCfgManager::new()
        //
    }
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

// NannigCfg ==================================================================

#[derive(Clone)]
pub struct NannigCfg {
    //
    second: NannigCfgGroup
    //
}

impl NannigCfg {
    fn new() -> Self {
        //
        Self {
            //profiles: HashMap::new(),
            second: NannigCfgGroup::new()
        }
        //
    }

    pub fn update_time(&mut self) {
        //
        //
    }
}

// NannigCfg Components =======================================================

#[derive(Clone)]
pub struct NannigCfgArc {
    //
    //
}

#[derive(Clone)]
pub struct NannigCfgGroup {
    //arc: Option<NannigCfgArc>,
    needle: Option<NannigCfgNeedle>
    //
}



impl NannigCfgGroup {
    pub fn new() -> Self {
        Self {
            //arc: None,
            needle: None
        }
    }
}

#[derive(Clone)]
pub struct NannigCfgNeedle {
    //
    //color: Color,
    curr_rot: f32,
    //length: u32,
    //negative: u32, //if the needle start before its rotational point
    //thickness: u32
    //
}