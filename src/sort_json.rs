use std::path::Path;

use anyhow::ensure;
use serde_jsonrc::Value;

use crate::SorterArgs;

pub fn sort_json(path: &Path, args: SorterArgs) -> anyhow::Result<()> {
    if args.verbose {
        eprintln!("JSON Sort {}",path.to_string_lossy());
    }

    ensure!(path.is_file());

    let data = std::fs::read(path)?;

    let mut json_value: Value = serde_jsonrc::from_slice(&data)?;

    sort_value(&mut json_value);

    let result = serde_jsonrc::to_vec_pretty(&json_value)?;

    if result != data {
        let _: Value = serde_jsonrc::from_slice(&result)?;

        if !args.noop {
            std::fs::write(path, result)?;
        }
    }

    Ok(())
}

pub fn sort_value(v: &mut Value) {
    match v {
        Value::Null => {},
        Value::Bool(_) => {},
        Value::Number(_) => {},
        Value::String(_) => {},
        Value::Array(a) => {
            for v in a {
                sort_value(v);
            }
        },
        Value::Object(o) => {
            for (_,v) in o.iter_mut() {
                sort_value(v);
            }
            o.sort_by_key_string();
        },
    }
}
