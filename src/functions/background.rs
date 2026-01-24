use std::collections::HashMap;
use crate::interpreter::Value;
use super::BuiltinFn;

#[cfg(windows)]
use winapi::um::wincon::FreeConsole;

pub fn register(map: &mut HashMap<String, BuiltinFn>) {
    map.insert("background".into(), background);
}

fn background(_args: Vec<Value>) -> Value {
    #[cfg(windows)]
    unsafe {
        FreeConsole();
        Value::Bool(false)
    }

    #[cfg(not(windows))]
    {
        Value::Error("background is Windows-only".into())
    }
}