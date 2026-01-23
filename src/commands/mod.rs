use crate::engine::Engine;

pub mod sleep;
pub mod print;

pub fn register_all(engine: &mut Engine) {
    sleep::register(engine);
    print::register(engine);
}