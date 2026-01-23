use std::collections::HashMap;
use crate::{interpreter::Context, ast::Node};

pub struct Engine {
    ctx: Context,
}

impl Engine {
    pub fn new() -> Self {
        let mut builtins = HashMap::new();
        crate::functions::register_all(&mut builtins);
        Self { ctx: Context::new(builtins) }
    }

    pub fn run(&self, nodes: Vec<Node>) -> Result<(), String> {
        self.ctx.run(&nodes)?;
        self.ctx.join_tasks()
    }
}