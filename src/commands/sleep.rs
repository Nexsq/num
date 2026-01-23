use crate::{
    engine::Engine,
    interpreter::Value,
};

pub fn register(engine: &mut Engine) {
    engine.register("sleep", |_, args| {
        if let Some(Value::Num(ms)) = args.get(0) {
            std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
        }
    });
}