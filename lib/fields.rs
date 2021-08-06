use std::fs;

use serde::Serialize;
use serde_json::Value;

/* ---------- */

#[derive(Debug)]
pub struct Field {
    name : String,
    value : String
}

impl Field {
    #[inline]
    pub fn new(name : &str, value : &str) -> Self {
        Self {
            name : String::from(name),
            value : String::from(value)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

/* ---------- */

pub type Fields = Vec<Field>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/* ---------- */

#[inline]
pub fn fields_from_file(json_path : &str) -> Result<Fields> {
    fields_from_str(&fs::read_to_string(json_path)?)
}

#[inline]
pub fn fields_from_str(content : &str) -> Result<Fields> {
    let values : Value = serde_json::from_str(content)?;
    Ok(generate_fields(&values, ""))
}

#[inline]
pub fn fields_from_struct<S: Serialize>(s: &S) -> Result<Fields> {
    let values : Value = serde_json::to_value(s)?;
    Ok(generate_fields(&values, ""))
}

/* ---------- */

fn generate_fields(field : &Value, prefix : &str) -> Fields {
    let mut ret = vec![];
    let clos = |(name, val) : (String, &Value)| {
        match val {
            Value::Array(_) | Value::Object(_) => ret.append(&mut generate_fields(val, &format!("{}_", name))),
            Value::String(inner) => ret.push(Field::new(&name.to_ascii_uppercase(), inner)),
            Value::Number(inner) => ret.push(Field::new(&name.to_ascii_uppercase(), &inner.to_string())),
            Value::Bool(inner) => ret.push(Field::new(&name.to_ascii_uppercase(), &inner.to_string())),
            Value::Null => ret.push(Field::new(&name.to_ascii_uppercase(), "null"))
        }
    };

    match field {
        Value::Array(arr) =>
            arr.iter()
                .enumerate()
                .map(|(ind, val)| {
                    (format!("{}{}", prefix, ind), val)
                })
                .for_each(clos),
        Value::Object(arr) =>
            arr.iter()
                .map(|(name, val)| {
                    (format!("{}{}", prefix, name), val)
                })
                .for_each(clos),
        _ => return vec![]
    };

    ret
}
