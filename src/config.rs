use crate::entity::{Person, Project};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::PathBuf};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Vault {
    #[serde(default = "default_version")]
    pub version: String,
    pub path: PathBuf,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub people: Vec<Person>,
}

fn default_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

impl Vault {
    pub fn create(path: String) -> Result<(), Box<dyn Error>> {
        let vault_path = PathBuf::from("mo.toml");

        if vault_path.exists() {
            println!("mo.toml already exists");
        } else {
            fs::create_dir_all(&path)?;
            let vault = Vault {
                version: env!("CARGO_PKG_VERSION").into(),
                path: PathBuf::from(&path),
                ..Default::default()
            };
            vault.save()?;
            println!("Created vault at: {}", path);
        };

        Ok(())
    }

    pub fn load() -> Result<Vault, Box<dyn Error>> {
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
        let vault: Vault = toml::from_str(&contents)?;
        Ok(vault)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let mut vault = self.clone();
        vault.people.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        vault.projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        
        let contents = toml::to_string_pretty(&vault)?;
        fs::write("mo.toml", contents)?;
        Ok(())
    }

    pub fn vault_path() -> Result<PathBuf, Box<dyn Error>> {
        let vault = Self::load()?;
        Ok(PathBuf::from(vault.path))
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
