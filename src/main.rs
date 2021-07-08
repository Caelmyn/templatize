use std::fs;

use serde_json::Value;

#[macro_use]
extern crate clap;
use clap::App;

pub mod templates;
use templates::TemplateList;

fn get_fields(fields : &Value, prefix : &str) -> Vec<(String, String)>{
    let mut ret = vec![];
    let clos = |(name, val) : (String, &Value)| {
        match val {
            val @ Value::Array(_) | val @ Value::Object(_) => { ret.append(&mut get_fields(val, &format!("{}_", name))) }
            val => ret.push((name.to_ascii_uppercase(), String::from(val.as_str().unwrap_or("null"))))
        }
    };

    match fields {
        Value::Array(arr) => { arr.iter().enumerate().map(|(ind, val)| { (format!("{}{}", prefix, ind), val) }).for_each(clos) }
        Value::Object(arr) => { arr.iter().map(|(name, val)| { (format!("{}{}", prefix, name), val )}).for_each(clos) }
        _ => return vec![]
    };

    ret
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yml = load_yaml!("cmd.yml");
    let args = App::from_yaml(yml).get_matches();

    let conf = args.value_of("config").unwrap_or_default();
    let json = args.value_of("json").unwrap();
    let parent = args.value_of("template-dir").unwrap_or_default();

    let fields = match fs::read_to_string(json) {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(ref json) => {
                    get_fields(json, "")
                }
                Err(err) => {
                    println!("{}", err);
                    return Err(err.into())
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            return Err(err.into());
        }
    };

    let tmpl = if !conf.is_empty() {
        TemplateList::from_json(conf).unwrap()
    } else {
        TemplateList::new(parent).unwrap()
    };

    if let Err(err) = tmpl.evaluate(&fields) {
        println!("{}", err);
        return Err(err)
    }

    Ok(())
}
