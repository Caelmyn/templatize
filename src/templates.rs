use std::fs;
use std::env;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use serde::Deserialize;
use regex::Regex;

#[derive(Deserialize, Debug)]
pub struct Template {
    src : String,
    dest : String,

    #[serde(default, alias = "eval_as_env")]
    eval_env : bool,
}

impl Template {
    pub fn new(name : &str, dest : &str) -> Self {
        Self {
            src : name.to_string(),
            dest : dest.to_string(),
            eval_env : false,
        }
    }

    pub fn evaluate(&self, fields : &[(String, String)], parent : &str) -> Result<(), Box<dyn std::error::Error>> {
        let parent = &format!("{}/{}", parent, self.src);
        let mut content = fs::read_to_string(parent)?;

        for (name, value) in fields {
            let re = Regex::new(&format!("%{}%", name))?;
            let value = if self.eval_env { env::vars().find(|(env_name, _)| { value == env_name }).map(|(_, val)| { val }).unwrap_or_else(|| value.clone()) } else { value.clone() };

            content = re.replace_all(&content, value).to_string();
        }

        let dest = Path::new(&self.dest);
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

        let dest = dest.join(&self.src);

        if cfg!(unix) {
            let mode = fs::metadata(parent)?.permissions().mode();
            let dest_file = fs::File::create(&dest)?;
            let mut dest_perm = dest_file.metadata()?.permissions();

            dest_perm.set_mode(mode);
            fs::set_permissions(&dest, dest_perm)?;
        }

        fs::write(dest, content)?;

        Ok(())
    }
}

/* -- */

#[derive(Deserialize, Debug, Default)]
pub struct TemplateList {
    parent : String,
    templates : Vec<Template>
}

impl TemplateList {
    pub fn new(path : &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut ret = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let dest = path.parent().unwrap().to_str().unwrap();
                let name = path.file_name().unwrap().to_str().unwrap();
                ret.push(Template::new(name, dest));
            }
        };

        Ok(Self {
            parent : String::from(path),
            templates : ret
        })
    }

    pub fn from_json(path : &str) -> Result<TemplateList, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn evaluate(&self, fields : &[(String, String)]) -> Result<(), Box<dyn std::error::Error>> {
        self.templates.iter().try_for_each(|temp| {
            temp.evaluate(fields, &self.parent)
        })
    }
}

/* -- */
