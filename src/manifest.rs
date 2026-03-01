use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::error::{self, ManifestError};

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub manifest_version: i32,
    pub author: String,
    pub version: String,
    pub description: String,
    pub tags: Vec<String>,
    pub ignored_files: Vec<String>,
}

pub fn init_manifest(path: impl AsRef<Path>) -> error::ManifestResult<()> {
    let test_manifest = Manifest {
        manifest_version: 1,
        author: String::new(),
        version: "0.0.1".to_string(),
        description: String::new(),
        tags: Vec::new(),
        ignored_files: vec!["tempo.toml".to_string()],
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                ManifestError::AlreadyExists
            } else {
                ManifestError::Io(e)
            }
        })?;

    let manifest_data = toml::to_string_pretty(&test_manifest)?;
    file.write_all(manifest_data.as_bytes())?;
    Ok(())
}

pub fn load_manifest(path: impl AsRef<Path>) -> error::ManifestResult<Manifest> {
    let contents = fs::read_to_string(path)?;
    let manifest = toml::from_str(&contents)?;
    Ok(manifest)
}
