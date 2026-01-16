use std::{collections::HashMap, env, fs, thread, time::Duration};

#[derive(Clone)]
enum Node {
    Command { name: String, args: Vec<String> },
    Loop { times: usize, body: Vec<Node> },
}

struct Command {
    arg_count: usize,
    run: fn(Vec<String>) -> Result<(), String>,
}

struct Engine {
    commands: HashMap<String, Command>,
}

impl Engine {
    fn new() -> Self {
        Self { commands: HashMap::new() }
    }

    fn add(&mut self, name: &str, arg_count: usize, run: fn(Vec<String>) -> Result<(), String>) {
        self.commands.insert(name.to_string(), Command { arg_count, run });
    }

    fn exec(&self, name: &str, args: Vec<String>) -> Result<(), String> {
        let cmd = self.commands.get(name).ok_or(format!("Unknown command '{}'", name))?;
        if args.len() != cmd.arg_count {
            return Err(format!("{} expects {} args", name, cmd.arg_count));
        }
        (cmd.run)(args)
    }
}

fn parse(lines: &mut std::iter::Peekable<std::str::Lines>) -> Result<Vec<Node>, String> {
    let mut nodes = Vec::new();

    while let Some(line) = lines.peek() {
        let line = line.trim();

        if line.is_empty() {
            lines.next();
            continue;
        }

        if line == "}" {
            lines.next();
            break;
        }

        if line.starts_with("loop") {
            let open = line.find('(').ok_or("Expected '(' in loop")?;
            let close = line.find(')').ok_or("Expected ')' in loop")?;
            let count: usize = line[open + 1..close].parse().map_err(|_| "Invalid loop count")?;

            if !line.ends_with('{') {
                return Err("Expected '{' after loop(...)".into());
            }

            lines.next();
            let body = parse(lines)?;
            nodes.push(Node::Loop { times: count, body });
        } else {
            let open = line.find('(').ok_or("Expected '('")?;
            let close = line.rfind(')').ok_or("Expected ')'")?;

            let name = line[..open].trim().to_string();
            let raw = &line[open + 1..close];

            let args = raw
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty())
                .collect();

            nodes.push(Node::Command { name, args });
            lines.next();
        }
    }

    Ok(nodes)
}

fn run(nodes: &[Node], engine: &Engine) -> Result<(), String> {
    for node in nodes {
        match node {
            Node::Command { name, args } => engine.exec(name, args.clone())?,
            Node::Loop { times, body } => {
                for _ in 0..*times {
                    run(body, engine)?;
                }
            }
        }
    }
    Ok(())
}

fn wait() {
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

fn main() {
    let file = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("No file provided");
            wait();
            return;
        }
    };

    let source = fs::read_to_string(file).expect("Failed to read file");

    let mut engine = Engine::new();

    engine.add("print", 1, |args| {
        println!("{}", args[0]);
        Ok(())
    });

    engine.add("sleep", 1, |args| {
        let ms: u64 = args[0].parse().map_err(|_| "Invalid number")?;
        thread::sleep(Duration::from_millis(ms));
        Ok(())
    });

    let mut lines = source.lines().peekable();
    let program = parse(&mut lines).expect("Parse error");

    if let Err(e) = run(&program, &engine) {
        println!("Runtime error: {}", e);
    }

    wait();
}
