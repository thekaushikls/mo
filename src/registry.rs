use crate::entity::{Person, Project};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registry {
    pub path: PathBuf,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub people: Vec<Person>,
}

impl Registry {
    pub fn create(path: String) -> Result<(), Box<dyn Error>> {
        let registry_path = PathBuf::from("mo.toml");

        if registry_path.exists() {
            println!("mo.toml already exists");
        } else {
            fs::create_dir_all(&path)?;
            let registry = Registry {
                path: PathBuf::from(&path),
                ..Default::default()
            };
            registry.save()?;
            println!("Created registry at: {}", path);
        };

        Ok(())
    }

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

    pub fn vault_path() -> Result<PathBuf, Box<dyn Error>> {
        let registry = Self::load()?;
        Ok(PathBuf::from(registry.path))
    }

    pub fn find_project(&self, name: &str) -> Option<&Project> {
        let needle = name.to_lowercase();
        self.projects.iter().find(|p| {
            p.name.to_lowercase() == needle || p.aliases.iter().any(|a| a.to_lowercase() == needle)
        })
    }

    pub fn find_person(&self, name: &str) -> Option<&Person> {
        let needle = name.to_lowercase();
        self.people.iter().find(|p| {
            p.name.to_lowercase() == needle || p.aliases.iter().any(|a| a.to_lowercase() == needle)
        })
    }
}
