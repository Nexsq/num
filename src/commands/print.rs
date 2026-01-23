use crate::{
    engine::Engine,
    interpreter::Value,
};

pub fn register(engine: &mut Engine) {
    engine.register("print", |_, args| {
        for v in args {
            match v {
                Value::Num(n) => print!("{n} "),
                Value::Str(s) => print!("{s} "),
                Value::Bool(b) => print!("{b} "),
            }
        }
        println!();
    });
}