use crate::functions::expect_arity;
use crate::interpreter::Value;
use rand::Rng;

pub fn register(map: &mut std::collections::HashMap<String, fn(Vec<Value>) -> Value>) {
    map.insert("random".into(), random);
}

fn random(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("random", &args, 2) {
        return e;
    }

    let (a, b) = match (&args[0], &args[1]) {
        (Value::Num(x), Value::Num(y)) => (*x, *y),
        _ => return Value::Error("random expects numbers".into()),
    };

    if a > b {
        return Value::Error("random range is invalid".into());
    }

    let mut rng = rand::rng();
    let n = rng.random_range(a..=b);
    Value::Num(n)
}
