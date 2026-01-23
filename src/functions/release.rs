use std::collections::HashMap;
use enigo::{
    Button,
    Direction::Release,
    Enigo, Key, Settings,
    Mouse, Keyboard,
};

use crate::interpreter::Value;
use super::BuiltinFn;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("release".into(), release);
}

fn release(args: Vec<Value>) -> Value {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let sym = match args.get(0) {
        Some(Value::Symbol(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };

    match sym {
        "LMB" => { let _ = enigo.button(Button::Left, Release); }
        "MMB" => { let _ = enigo.button(Button::Middle, Release); }
        "RMB" => { let _ = enigo.button(Button::Right, Release); }

        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            let _ = enigo.key(Key::Unicode(c), Release);
        }

        "Enter" => { let _ = enigo.key(Key::Return, Release); }
        "Space" => { let _ = enigo.key(Key::Space, Release); }
        "Tab" => { let _ = enigo.key(Key::Tab, Release); }
        "Escape" => { let _ = enigo.key(Key::Escape, Release); }

        "Ctrl" => { let _ = enigo.key(Key::Control, Release); }
        "Alt" => { let _ = enigo.key(Key::Alt, Release); }
        "Shift" => { let _ = enigo.key(Key::Shift, Release); }

        _ => {}
    }

    Value::Bool(false)
}