use std::{error::Error, fs, path::Path, string::String};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registry {
    #[serde(default)]
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "active".into()
}

impl Registry {
    pub fn load(path: &Path) -> Result<Registry, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let registry: Registry = toml::from_str(&contents)?;
        Ok(registry)
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }
    
    pub fn find_project(&self, name: &str) -> Option<&Project> {
        let needle = name.to_lowercase();
        self.projects.iter().find(|p| {
            p.name.to_lowercase() == needle
                || p.aliases.iter().any(|a| a.to_lowercase() == needle)
        })
    }
}