use scrap::{Capturer, Display};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::time::Duration;

use super::BuiltinFn;
use crate::functions::expect_arity;
use crate::interpreter::Value;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("color".into(), color);
}

fn parse_hex(s: &str) -> Option<(i64, i64, i64)> {
    let h = s.trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let r = i64::from_str_radix(&h[0..2], 16).ok()?;
    let g = i64::from_str_radix(&h[2..4], 16).ok()?;
    let b = i64::from_str_radix(&h[4..6], 16).ok()?;
    Some((r, g, b))
}

fn color(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("color", &args, 4) {
        return e;
    }

    let hex = match args.get(0) {
        Some(Value::Symbol(s)) | Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error("color expects hex string".into()),
    };

    let (er, eg, eb) = match parse_hex(hex) {
        Some(v) => v,
        None => return Value::Error("invalid hex color".into()),
    };

    let x = match args.get(1) {
        Some(Value::Num(n)) if *n >= 0 => *n as usize,
        _ => return Value::Error("color expects number x".into()),
    };

    let y = match args.get(2) {
        Some(Value::Num(n)) if *n >= 0 => *n as usize,
        _ => return Value::Error("color expects number y".into()),
    };

    let tol = match args.get(3) {
        Some(Value::Num(n)) if *n >= 0 => *n,
        _ => return Value::Error("color expects tolerance".into()),
    };

    let display = match Display::primary() {
        Ok(d) => d,
        Err(_) => return Value::Bool(false),
    };

    let mut capturer = match Capturer::new(display) {
        Ok(c) => c,
        Err(_) => return Value::Bool(false),
    };

    let width = capturer.width();
    let height = capturer.height();

    if x >= width || y >= height {
        return Value::Bool(false);
    }

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let stride = width * 4;
                let idx = y * stride + x * 4;
                if idx + 2 >= frame.len() {
                    return Value::Bool(false);
                }
                let b = frame[idx] as i64;
                let g = frame[idx + 1] as i64;
                let r = frame[idx + 2] as i64;
                return Value::Bool(
                    (r - er).abs() <= tol && (g - eg).abs() <= tol && (b - eb).abs() <= tol,
                );
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(1));
            }
            Err(_) => return Value::Bool(false),
        }
    }
}
