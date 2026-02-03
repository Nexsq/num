use scrap::{Capturer, Display};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::time::Duration;

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("get_color".into(), get_color);
}

fn get_color(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("get_color", &args, 2) {
        return e;
    }

    let x = match args.get(0) {
        Some(Value::Num(n)) if *n >= 0 => *n as usize,
        _ => return Value::Error("get_color expects number x".into()),
    };

    let y = match args.get(1) {
        Some(Value::Num(n)) if *n >= 0 => *n as usize,
        _ => return Value::Error("get_color expects number y".into()),
    };

    let display = match Display::primary() {
        Ok(d) => d,
        Err(_) => return Value::Error("failed to read pixel".into()),
    };

    let mut capturer = match Capturer::new(display) {
        Ok(c) => c,
        Err(_) => return Value::Error("failed to read pixel".into()),
    };

    let width = capturer.width();
    let height = capturer.height();

    if x >= width || y >= height {
        return Value::Error("failed to read pixel".into());
    }

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let stride = width * 4;
                let idx = y * stride + x * 4;
                if idx + 2 >= frame.len() {
                    return Value::Error("failed to read pixel".into());
                }
                let b = frame[idx];
                let g = frame[idx + 1];
                let r = frame[idx + 2];
                return Value::Str(format!("#{:02x}{:02x}{:02x}", r, g, b));
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(1));
            }
            Err(_) => return Value::Error("failed to read pixel".into()),
        }
    }
}
