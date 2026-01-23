use std::collections::HashMap;
use enigo::{
    Button,
    Direction::Press,
    Enigo, Key, Settings,
    Mouse, Keyboard,
};

use crate::interpreter::Value;
use super::BuiltinFn;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("press".into(), press);
}

fn press(args: Vec<Value>) -> Value {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let sym = match args.get(0) {
        Some(Value::Symbol(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };

    match sym {
        "LMB" => { let _ = enigo.button(Button::Left, Press); }
        "MMB" => { let _ = enigo.button(Button::Middle, Press); }
        "RMB" => { let _ = enigo.button(Button::Right, Press); }

        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            let _ = enigo.key(Key::Unicode(c), Press);
        }

        "Enter" => { let _ = enigo.key(Key::Return, Press); }
        "Space" => { let _ = enigo.key(Key::Space, Press); }
        "Tab" => { let _ = enigo.key(Key::Tab, Press); }
        "Escape" => { let _ = enigo.key(Key::Escape, Press); }

        "Ctrl" => { let _ = enigo.key(Key::Control, Press); }
        "Alt" => { let _ = enigo.key(Key::Alt, Press); }
        "Shift" => { let _ = enigo.key(Key::Shift, Press); }

        _ => {}
    }

    Value::Bool(false)
}