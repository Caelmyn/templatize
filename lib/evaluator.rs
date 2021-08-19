use std::fs;
use std::path::Path;

use serde::Deserialize;

use super::{
    Template,
    Templates,
    Field
};

/* -------- */

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Evaluator {
    default_relative_dir : String,
    default_dest_dir : String,

    #[serde(alias = "files")]
    templates : Templates
}

impl Evaluator {
    pub fn new(path : &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut ret = Evaluator::default();

        ret.get_files_in_dir(Path::new(path))?;
        Ok(ret)
    }

    pub fn from_json(path : &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut ret : Self = serde_json::from_str(&fs::read_to_string(path)?)?;

        if ret.templates.is_empty() {
            if ret.default_relative_dir.is_empty() {
                return Ok(ret)
            }

            let clone = ret.default_relative_dir.clone();
            ret.get_files_in_dir(Path::new(&clone))?;
        }

        for template in ret.templates.iter_mut() {
            template.set_relative_dir(&ret.default_relative_dir);
            template.set_dest_dir(&ret.default_dest_dir);
        }

        Ok(ret)
    }

    pub fn evaluate(&self, fields : &[Field]) -> Result<(), Box<dyn std::error::Error>> {
        for template in &self.templates {
            template.evaluate(fields)?
        }

        Ok(())
    }
}

/* -------- */

impl Evaluator {
    fn get_files_in_dir(&mut self, path : &Path) -> Result<(), Box<dyn std::error::Error>> {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;

            if entry.file_type()?.is_dir() {
                self.get_files_in_dir(entry.path().as_path())?
            } else if entry.file_type()?.is_file() {
                self.templates.push(Template::from_path(entry.path().as_path())?)
            }
        };

        Ok(())
    }
}
