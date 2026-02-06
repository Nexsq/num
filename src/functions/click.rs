use enigo::{Button, Direction::Click, Enigo, Key, Keyboard, Mouse, Settings};
use std::collections::HashMap;

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("click".into(), click);
}

fn click(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("click", &args, 1) {
        return e;
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let sym_owned;
    let sym = match args.get(0) {
        Some(Value::Key(s)) => s.as_str(),
        Some(Value::Str(s)) => s.as_str(),
        Some(Value::Num(n)) => {
            sym_owned = n.to_string();
            &sym_owned
        }
        _ => return Value::Error("press expects a key".into()),
    };

    match sym {
        "LMB" => {
            let _ = enigo.button(Button::Left, Click);
        }
        "MMB" => {
            let _ = enigo.button(Button::Middle, Click);
        }
        "RMB" => {
            let _ = enigo.button(Button::Right, Click);
        }
        "MB4" => {
            let _ = enigo.button(Button::Back, Click);
        }
        "MB5" => {
            let _ = enigo.button(Button::Forward, Click);
        }

        s if s.len() == 1 && s.chars().next().unwrap().is_ascii_digit() => {
            let k = match s {
                "0" => Key::Num0,
                "1" => Key::Num1,
                "2" => Key::Num2,
                "3" => Key::Num3,
                "4" => Key::Num4,
                "5" => Key::Num5,
                "6" => Key::Num6,
                "7" => Key::Num7,
                "8" => Key::Num8,
                "9" => Key::Num9,
                _ => unreachable!(),
            };
            let _ = enigo.key(k, Click);
        }

        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            let _ = enigo.key(Key::Unicode(c), Click);
        }

        "Enter" | "Return" => {
            let _ = enigo.key(Key::Return, Click);
        }
        "Space" => {
            let _ = enigo.key(Key::Space, Click);
        }
        "Tab" => {
            let _ = enigo.key(Key::Tab, Click);
        }
        "Esc" | "Escape" => {
            let _ = enigo.key(Key::Escape, Click);
        }
        "Backspace" => {
            let _ = enigo.key(Key::Backspace, Click);
        }

        "Ctrl" | "Control" => {
            let _ = enigo.key(Key::Control, Click);
        }
        "Alt" => {
            let _ = enigo.key(Key::Alt, Click);
        }
        "Shift" => {
            let _ = enigo.key(Key::Shift, Click);
        }
        "Super" | "Meta" => {
            let _ = enigo.key(Key::Meta, Click);
        }
        "CapsLock" | "Caps" => {
            let _ = enigo.key(Key::CapsLock, Click);
        }

        "Insert" => {
            let _ = enigo.key(Key::Insert, Click);
        }
        "Delete" | "Del" => {
            let _ = enigo.key(Key::Delete, Click);
        }
        "Home" => {
            let _ = enigo.key(Key::Home, Click);
        }
        "End" => {
            let _ = enigo.key(Key::End, Click);
        }
        "PageUp" | "PgUp" => {
            let _ = enigo.key(Key::PageUp, Click);
        }
        "PageDown" | "PgDown" => {
            let _ = enigo.key(Key::PageDown, Click);
        }

        "Up" => {
            let _ = enigo.key(Key::UpArrow, Click);
        }
        "Down" => {
            let _ = enigo.key(Key::DownArrow, Click);
        }
        "Left" => {
            let _ = enigo.key(Key::LeftArrow, Click);
        }
        "Right" => {
            let _ = enigo.key(Key::RightArrow, Click);
        }

        "F1" => {
            let _ = enigo.key(Key::F1, Click);
        }
        "F2" => {
            let _ = enigo.key(Key::F2, Click);
        }
        "F3" => {
            let _ = enigo.key(Key::F3, Click);
        }
        "F4" => {
            let _ = enigo.key(Key::F4, Click);
        }
        "F5" => {
            let _ = enigo.key(Key::F5, Click);
        }
        "F6" => {
            let _ = enigo.key(Key::F6, Click);
        }
        "F7" => {
            let _ = enigo.key(Key::F7, Click);
        }
        "F8" => {
            let _ = enigo.key(Key::F8, Click);
        }
        "F9" => {
            let _ = enigo.key(Key::F9, Click);
        }
        "F10" => {
            let _ = enigo.key(Key::F10, Click);
        }
        "F11" => {
            let _ = enigo.key(Key::F11, Click);
        }
        "F12" => {
            let _ = enigo.key(Key::F12, Click);
        }

        _ => return Value::Error("invalid click target".into()),
    }

    Value::Bool(false)
}
