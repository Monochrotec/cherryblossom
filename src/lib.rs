use std::{collections::HashSet, fs, ops::Deref, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum BlossomFileErrorType {
    Io(#[from] std::io::Error),
    TomlSerialization(#[from] toml::ser::Error),
    TomlDeserialization(#[from] toml::de::Error),
}

/// A struct containing all of the needed packages.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageList(HashSet<PackageSpec>);

impl PackageList {
    pub fn from_lockfile<P: AsRef<Path>>(path: P) -> Result<Self, BlossomFileErrorType> {
        let contents = fs::read(path)?;

        let output: Self = toml::from_slice(&contents)?;

        Ok(output)
    }
}

impl Deref for PackageList {
    type Target = HashSet<PackageSpec>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A struct containing information about a specific package.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PackageSpec {
    pub name: String,
    pub version: String,
}
