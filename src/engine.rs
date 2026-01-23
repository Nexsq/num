use std::collections::HashMap;
use crate::{interpreter::{Context, Value}, ast::Node};

pub type CommandFn = fn(&mut Context, Vec<Value>);

pub struct Engine {
    ctx: Context,
    commands: HashMap<String, CommandFn>,
}

impl Engine {
    pub fn new() -> Self {
        let mut e = Self {
            ctx: Context::new(),
            commands: HashMap::new(),
        };

        crate::commands::register_all(&mut e);

        e
    }

    pub fn register(&mut self, name: &str, f: CommandFn) {
        self.commands.insert(name.to_string(), f);
    }

    pub fn run(&mut self, nodes: Vec<Node>) -> Result<(), String> {
        self.ctx.run_with_commands(&nodes, &self.commands)
    }
}