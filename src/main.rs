#[macro_use]
extern crate clap;
use clap::App;

use templatize::Evaluator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yml = load_yaml!("cmd.yml");
    let args = App::from_yaml(yml).get_matches();

    let conf = args.value_of("config").unwrap();
    let json = args.value_of("json").unwrap();

    let fields = templatize::fields_from_file(json)?;
    let eval = Evaluator::from_json(conf)?;

    eval.evaluate(&fields)?;

    Ok(())
}
