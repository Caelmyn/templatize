use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use regex::Regex;
use serde::Deserialize;

use super::Field;

/* ---------- */

#[derive(Deserialize, Default, Debug)]
pub struct Template {
    #[serde(default)]
    relative_dir : String,

    file_name : String,

    #[serde(default)]
    dest_dir : String,

    #[serde(default)]
    alt_dest_name : Option<String>,

    #[serde(skip)]
    data : String
}

impl Template {
    pub fn new(file_name : &str, relative_dir : &str, dest_dir : Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let dest_dir = String::from(dest_dir.unwrap_or_default());

        let data = fs::read_to_string(format!("{}/{}", relative_dir, file_name))?;

        Ok(Self {
            relative_dir : String::from(relative_dir),
            file_name : String::from(file_name),
            dest_dir,
            alt_dest_name : None,
            data
        })
    }

    pub fn from_path(path : &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let parent = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_str()
            .unwrap_or_default();

        Self::new(file_name, parent, None)
    }

    #[inline]
    pub fn set_relative_dir(&mut self, relative_dir : &str) {
        if self.relative_dir.is_empty() {
            self.relative_dir = String::from(relative_dir)
        }

        self.data = fs::read_to_string(format!("{}/{}", self.relative_dir, self.file_name)).unwrap_or_default();
    }

    #[inline]
    pub fn set_dest_dir(&mut self, dest_dir : &str) {
        if self.dest_dir.is_empty() {
            self.dest_dir = String::from(dest_dir)
        }
    }

    pub fn evaluate(&self, fields : &[Field]) -> Result<(), Box<dyn std::error::Error>> {
        if self.data.is_empty() {
            return Ok(())
        }

        let mut content = self.data.clone();

        for field in fields {
            let re = Regex::new(&format!("%{}%", field.name()))?;
            content = re.replace_all(&content, field.value()).to_string();
        }

        let dest_dir = Path::new(&self.dest_dir);
        if !dest_dir.exists() {
            fs::create_dir_all(dest_dir)?;
        }

        let dest_file = dest_dir.join(&self.alt_dest_name.as_ref().unwrap_or(&self.file_name));

        #[cfg(unix)] {
            let source_file_path = format!("{}/{}", self.relative_dir, self.file_name);
            let mode = fs::metadata(source_file_path)?.permissions().mode();
            let file = fs::File::create(&dest_file)?;
            let mut dest_perm = file.metadata()?.permissions();

            dest_perm.set_mode(mode);
            fs::set_permissions(&dest_file, dest_perm)?;
        }

        fs::write(dest_file, content)?;

        Ok(())
    }
}

/* ---------- */

pub type Templates = Vec<Template>;
