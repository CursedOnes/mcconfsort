use std::path::Path;

use anyhow::ensure;
use toml_edit::*;

pub fn sort_toml(path: &Path) -> anyhow::Result<()> {
    eprintln!("TOML Sort {}",path.to_string_lossy());

    ensure!(path.is_file());

    let data = std::fs::read_to_string(path)?;

    let mut doc = data.parse::<Document>()?;

    sort_item(doc.as_item_mut());

    let result = format!("{}",&doc);

    let _ = result.parse::<Document>()?;

    if result != data {
        std::fs::write(path, result)?;
    }

    Ok(())
}

fn sort_item(i: &mut Item) {
    match i {
        Item::None => {},
        Item::Value(v) => sort_value(v),
        Item::Table(t) => {
            t.remove_position();
            for (_,v) in t.iter_mut() {
                sort_item(v);
            }
            t.sort_values();
        },
        Item::ArrayOfTables(t) => {
            for t in t.iter_mut() {
                t.remove_position();
                for (_,v) in t.iter_mut() {
                    sort_item(v);
                }
                t.sort_values();
            }
        },
    }
}

fn sort_value(v: &mut Value) {
    match v {
        Value::String(_) => {},
        Value::Integer(_) => {},
        Value::Float(_) => {},
        Value::Boolean(_) => {},
        Value::Datetime(_) => {},
        Value::Array(a) => {
            for v in a.iter_mut() {
                sort_value(v);
            }
            //let all_tables = a.iter().all(|v| matches!(v,Value::Table));
        },
        Value::InlineTable(t) => {
            for (_,v) in t.iter_mut() {
                sort_value(v);
            }
            t.sort_values();
        },
    }
}
