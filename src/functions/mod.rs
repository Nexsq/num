use std::collections::HashMap;
use crate::interpreter::Value;

pub mod print;
pub mod sleep;
pub mod click;
pub mod press;
pub mod release;
pub mod scroll;
pub mod mouse;
pub mod string;

pub type BuiltinFn = fn(Vec<Value>) -> Value;

pub fn register_all(map: &mut HashMap<String, BuiltinFn>) {
    print::register(map);
    sleep::register(map);
    click::register(map);
    press::register(map);
    release::register(map);
    scroll::register(map);
    mouse::register(map);
    string::register(map);
}