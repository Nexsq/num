use std::collections::HashMap;
use enigo::{
    Enigo, Settings,
    Keyboard,
};

use crate::interpreter::Value;
use super::BuiltinFn;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("string".into(), string);
}

fn string(args: Vec<Value>) -> Value {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let text = match args.get(0) {
        Some(Value::Str(s)) => s,
        _ => return Value::Bool(false),
    };

    let _ = enigo.text(text);

    Value::Bool(false)
}