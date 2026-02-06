use crate::functions::expect_arity;
use crate::interpreter::Value;
use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn register(map: &mut std::collections::HashMap<String, fn(Vec<Value>) -> Value>) {
    map.insert("key".into(), key);
}

fn key(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("key", &args, 1) {
        return e;
    }

    let key_name = match &args[0] {
        Value::Key(s) => s.clone(),
        Value::Str(s) => s.clone(),
        Value::Num(n) => n.to_string(),
        _ => return Value::Error("key expects a key name".into()),
    };

    let device = DeviceState::new();
    let pressed = match key_name.as_str() {
        "Ctrl" | "Control" => {
            device.get_keys().contains(&Keycode::LControl)
                || device.get_keys().contains(&Keycode::RControl)
        }
        "LCtrl" | "LControl" => device.get_keys().contains(&Keycode::LControl),
        "RCtrl" | "RControl" => device.get_keys().contains(&Keycode::RControl),
        "Shift" => {
            device.get_keys().contains(&Keycode::LShift)
                || device.get_keys().contains(&Keycode::RShift)
        }
        "LShift" => device.get_keys().contains(&Keycode::LShift),
        "RShift" => device.get_keys().contains(&Keycode::RShift),
        "Alt" => {
            device.get_keys().contains(&Keycode::LAlt) || device.get_keys().contains(&Keycode::RAlt)
        }
        "LAlt" => device.get_keys().contains(&Keycode::LAlt),
        "RAlt" => device.get_keys().contains(&Keycode::RAlt),
        "Super" | "Meta" => {
            device.get_keys().contains(&Keycode::LMeta)
                || device.get_keys().contains(&Keycode::RMeta)
        }

        "Enter" | "Return" => device.get_keys().contains(&Keycode::Enter),
        "Space" => device.get_keys().contains(&Keycode::Space),
        "Tab" => device.get_keys().contains(&Keycode::Tab),
        "Esc" | "Escape" => device.get_keys().contains(&Keycode::Escape),
        "Backspace" => device.get_keys().contains(&Keycode::Backspace),

        "Up" => device.get_keys().contains(&Keycode::Up),
        "Down" => device.get_keys().contains(&Keycode::Down),
        "Left" => device.get_keys().contains(&Keycode::Left),
        "Right" => device.get_keys().contains(&Keycode::Right),

        "F1" => device.get_keys().contains(&Keycode::F1),
        "F2" => device.get_keys().contains(&Keycode::F2),
        "F3" => device.get_keys().contains(&Keycode::F3),
        "F4" => device.get_keys().contains(&Keycode::F4),
        "F5" => device.get_keys().contains(&Keycode::F5),
        "F6" => device.get_keys().contains(&Keycode::F6),
        "F7" => device.get_keys().contains(&Keycode::F7),
        "F8" => device.get_keys().contains(&Keycode::F8),
        "F9" => device.get_keys().contains(&Keycode::F9),
        "F10" => device.get_keys().contains(&Keycode::F10),
        "F11" => device.get_keys().contains(&Keycode::F11),
        "F12" => device.get_keys().contains(&Keycode::F12),

        k if k.len() == 1 => {
            let c = k.chars().next().unwrap().to_ascii_lowercase();
            device.get_keys().iter().any(|kc| match kc {
                Keycode::A => c == 'a',
                Keycode::B => c == 'b',
                Keycode::C => c == 'c',
                Keycode::D => c == 'd',
                Keycode::E => c == 'e',
                Keycode::F => c == 'f',
                Keycode::G => c == 'g',
                Keycode::H => c == 'h',
                Keycode::I => c == 'i',
                Keycode::J => c == 'j',
                Keycode::K => c == 'k',
                Keycode::L => c == 'l',
                Keycode::M => c == 'm',
                Keycode::N => c == 'n',
                Keycode::O => c == 'o',
                Keycode::P => c == 'p',
                Keycode::Q => c == 'q',
                Keycode::R => c == 'r',
                Keycode::S => c == 's',
                Keycode::T => c == 't',
                Keycode::U => c == 'u',
                Keycode::V => c == 'v',
                Keycode::W => c == 'w',
                Keycode::X => c == 'x',
                Keycode::Y => c == 'y',
                Keycode::Z => c == 'z',
                Keycode::Key0 => c == '0',
                Keycode::Key1 => c == '1',
                Keycode::Key2 => c == '2',
                Keycode::Key3 => c == '3',
                Keycode::Key4 => c == '4',
                Keycode::Key5 => c == '5',
                Keycode::Key6 => c == '6',
                Keycode::Key7 => c == '7',
                Keycode::Key8 => c == '8',
                Keycode::Key9 => c == '9',
                _ => false,
            })
        }

        _ => false,
    };

    Value::Bool(pressed)
}
