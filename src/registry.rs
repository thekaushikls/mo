use std::{error::Error, fs, path::PathBuf, string::String};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VaultConfig {
    pub path: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registry {
    pub vault: VaultConfig,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub people: Vec<Person>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default = "default_status")]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
}

fn default_status() -> String {
    "active".into()
}

impl Registry {
    pub fn load() -> Result<Registry, Box<dyn Error>> {
        let local = PathBuf::from("mo.toml");
        let global = dirs::config_dir()
            .map(|d| d.join("mo").join("mo.toml"))
            .unwrap_or_default();

        let path = if local.exists() {
            local
        } else if global.exists() {
            global
        } else {
            return Err("No mo.toml found. Run `mo init` first.".into());
        };

        let contents = fs::read_to_string(&path)?;
        let registry: Registry = toml::from_str(&contents)?;
        Ok(registry)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string_pretty(self)?;
        fs::write("mo.toml", contents)?;
        Ok(())
    }
    
    pub fn find_project(&self, name: &str) -> Option<&Project> {
        let needle = name.to_lowercase();
        self.projects.iter().find(|p| {
            p.name.to_lowercase() == needle
                || p.aliases.iter().any(|a| a.to_lowercase() == needle)
        })
    }

    pub fn find_person(&self, name: &str) -> Option<&Person> {
        let needle = name.to_lowercase();
        self.people.iter().find(|p| {
            p.name.to_lowercase() == needle
                || p.aliases.iter().any(|a| a.to_lowercase() == needle)
        })
    }
}