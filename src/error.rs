use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Failed to create manifest: {0}")]
    ManifestError(#[from] ManifestError),

    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("New template error: {0}")]
    NewTemplateError(#[from] NewTemplateError),

    #[error("Delete template error: {0}")]
    DeleteTemplateError(#[from] DeleteTemplateError),

    #[error("New project error: {0}")]
    NewProjectError(#[from] NewProjectError),
}

pub type CliResult<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Toml Serialization error")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Toml deserialization error")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Manifest already exists")]
    AlreadyExists,
}

pub type ManifestResult<T> = std::result::Result<T, ManifestError>;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),
}

pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

#[derive(Error, Debug)]
pub enum NewTemplateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),

    #[error("Manifest read error: {0}")]
    Manifest(#[from] ManifestError),

    #[error("Collect files error: {0}")]
    CollectFiles(#[from] walkdir::Error),

    #[error("Template with name already exists")]
    AlreadyExists,

    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
}

pub type NewTemplateResult<T> = std::result::Result<T, NewTemplateError>;

#[derive(Error, Debug)]
pub enum DeleteTemplateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Template does not exists")]
    NoTemplateFound,

    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
}

pub type DeleteTemplateResult<T> = std::result::Result<T, DeleteTemplateError>;

#[derive(Error, Debug)]
pub enum NewProjectError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Template does not exists")]
    NoTemplateFound,

    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
}

pub type NewProjectResult<T> = std::result::Result<T, NewProjectError>;
