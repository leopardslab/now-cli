use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NowConfig {
    #[serde(default)]
    pub commands: BTreeMap<String, Command>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Command {
    Single(Step),
    Pipeline { steps: Vec<Step> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Step {
    #[serde(default)]
    pub image: Option<String>,
    pub run: String,
    #[serde(default)]
    pub host: bool,
}

pub fn load_from(path: impl AsRef<Path>) -> Result<Option<NowConfig>> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(None);
    }

    let raw =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let config = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed to parse {}", path.display()))?;

    Ok(Some(config))
}
