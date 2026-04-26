use crate::registry::Registry;
use serde::{Deserialize, Serialize};
use std::error::Error;

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

impl Project {
    pub fn add(name: String, aliases: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut registry = Registry::load()?;

        if registry.find_project(&name).is_some() {
            println!("Project {} already exists.", name);
            return Ok(());
        }

        let new_project = Project {
            name: name.clone(),
            aliases,
            status: "active".into(),
        };
        registry.projects.push(new_project);
        registry.save()?;

        println!("Added project `{}`.", name);
        Ok(())
    }

    pub fn list() -> Result<(), Box<dyn Error>> {
        let registry = Registry::load()?;

        if registry.projects.is_empty() {
            println!("No projects");
            return Ok(());
        }

        for p in &registry.projects {
            println!("    {} ({})", p.name, p.status);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
}

impl Person {
    pub fn add(name: String, aliases: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut registry = Registry::load()?;

        if registry.find_person(&name).is_some() {
            println!("Person {} already exists.", name);
            return Ok(());
        }

        let new_person = Person {
            name: name.clone(),
            aliases,
        };
        registry.people.push(new_person);
        registry.save()?;

        println!("Added person `{}`.", name);
        Ok(())
    }

    pub fn list() -> Result<(), Box<dyn Error>> {
        let registry = Registry::load()?;

        if registry.people.is_empty() {
            println!("No people");
            return Ok(());
        }

        for person in &registry.people {
            println!("    {}", person.name);
        }

        Ok(())
    }
}
