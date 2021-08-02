use std::path::PathBuf;
use serde::Deserialize;

use crate::config::load_configuration;
use super::WindowModes;

#[derive(Deserialize)]
pub struct WindowConfiguration {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub window_mode: WindowModes,
}

impl WindowConfiguration {
    pub fn new() -> Self {
        match load_configuration(PathBuf::from("./window.config")) {
            Ok(config) => config,
            Err(_) => default_window_configuration(),
        }
    }
}

fn default_window_configuration() -> WindowConfiguration {
    WindowConfiguration {
        title: String::from("default window"),
        width: 1920,
        height: 1080,
        window_mode: WindowModes::Window,
    }
}