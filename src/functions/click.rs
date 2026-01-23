use std::collections::HashMap;
use enigo::{
    Button,
    Direction::Click,
    Enigo, Key, Settings,
    Mouse, Keyboard,
};

use crate::interpreter::Value;
use super::BuiltinFn;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("click".into(), click);
}

fn click(args: Vec<Value>) -> Value {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let sym = match args.get(0) {
        Some(Value::Symbol(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };

    match sym {
        "LMB" => { let _ = enigo.button(Button::Left, Click); }
        "MMB" => { let _ = enigo.button(Button::Middle, Click); }
        "RMB" => { let _ = enigo.button(Button::Right, Click); }

        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            let _ = enigo.key(Key::Unicode(c), Click);
        }

        "Enter" => { let _ = enigo.key(Key::Return, Click); }
        "Space" => { let _ = enigo.key(Key::Space, Click); }
        "Tab" => { let _ = enigo.key(Key::Tab, Click); }
        "Escape" => { let _ = enigo.key(Key::Escape, Click); }

        "Ctrl" => { let _ = enigo.key(Key::Control, Click); }
        "Alt" => { let _ = enigo.key(Key::Alt, Click); }
        "Shift" => { let _ = enigo.key(Key::Shift, Click); }

        _ => {}
    }

    Value::Bool(false)
}