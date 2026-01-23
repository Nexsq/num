use std::{collections::HashMap, thread, time::Duration};
use crate::interpreter::Value;
use super::BuiltinFn;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("sleep".into(), sleep);
}

fn sleep(args: Vec<Value>) -> Value {
    let ms = match args.get(0) {
        Some(Value::Num(n)) => *n,
        _ => return Value::Bool(false),
    };

    if ms > 0 {
        thread::sleep(Duration::from_millis(ms as u64));
    }

    Value::Bool(false)
}