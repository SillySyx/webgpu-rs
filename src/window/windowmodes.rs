use serde::Deserialize;

#[derive(Deserialize)]
pub enum WindowModes {
    Window,
    Borderless,
    Exclusive,
}