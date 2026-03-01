use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

use crate::{
    config::TemplatesConfig,
    error::{self, NewTemplateError, NewTemplateResult},
    manifest::{Manifest, load_manifest},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub manifest_version: i32,
    pub author: String,
    pub version: String,
    pub description: String,
    pub tags: Vec<String>,
    pub files: Vec<String>,
}

pub fn create_new_template(
    name: &str,
    manifest_path: impl AsRef<Path>,
    templates_config: &mut TemplatesConfig,
    templates_dir: &Path,
    app_name: &str,
) -> error::NewTemplateResult<()> {
    for template in &templates_config.templates {
        if template.name == name {
            return Err(NewTemplateError::AlreadyExists);
        }
    }

    let manifest = load_manifest(manifest_path)?;
    let files = collect_files(".", &manifest)?;
    let files_strings = files
        .clone()
        .into_iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();

    let template = Template {
        name: name.to_string(),
        manifest_version: manifest.manifest_version,
        author: manifest.author,
        version: manifest.version,
        description: manifest.description,
        tags: manifest.tags,
        files: files_strings,
    };

    let new_template_dir = templates_dir.join(name);

    for file in files {
        let old_file_path = &file;
        let new_file_path = new_template_dir.join(old_file_path);

        if let Some(parent) = new_file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(old_file_path, new_file_path)?;
    }

    templates_config.templates.push(template);
    templates_config.write_to_file(app_name)?;

    Ok(())
}

fn should_enter(entry: &DirEntry, ignored: &[String]) -> bool {
    let name = entry.file_name().to_string_lossy();
    let name = name.as_ref();
    !ignored.iter().any(|s| s == name)
}

fn collect_files(path: impl AsRef<Path>, manifest: &Manifest) -> NewTemplateResult<Vec<PathBuf>> {
    let base = path.as_ref();
    let mut files = Vec::new();
    let walker = WalkDir::new(base).into_iter();
    for entry in walker.filter_entry(|e| should_enter(e, &manifest.ignored_files)) {
        let entry = entry?;

        if entry.file_type().is_file() {
            files.push(entry.into_path());
        }
    }
    Ok(files)
}

pub fn delete_template(
    name: &str,
    templates_config: &mut TemplatesConfig,
    templates_dir: &Path,
    app_name: &str,
) -> error::DeleteTemplateResult<()> {
    if !templates_config
        .clone()
        .templates
        .into_iter()
        .any(|t| t.name == name)
    {
        return Err(error::DeleteTemplateError::NoTemplateFound);
    }

    let template_dir = templates_dir.join(name);
    fs::remove_dir_all(template_dir)?;
    if let Some(pos) = templates_config
        .templates
        .iter()
        .position(|t| t.name == name)
    {
        templates_config.templates.remove(pos);
    }
    templates_config.write_to_file(app_name)?;

    Ok(())
}

pub fn create_project(
    name: &str,
    templates_config: &TemplatesConfig,
    templates_dir: &Path,
) -> error::NewProjectResult<()> {
    if !templates_config
        .clone()
        .templates
        .into_iter()
        .any(|t| t.name == name)
    {
        return Err(error::NewProjectError::NoTemplateFound);
    }

    let template = templates_config
        .clone()
        .templates
        .into_iter()
        .find(|t| t.name == name)
        .ok_or(error::NewProjectError::NoTemplateFound)?;

    let template_dir = templates_dir.join(&name);

    let files: Vec<PathBuf> = template
        .files
        .clone()
        .into_iter()
        .map(PathBuf::from)
        .collect();

    for file in files {
        let old_file_path = template_dir.join(&file);

        if let Some(parent) = file.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(old_file_path, file)?;
    }

    Ok(())
}

pub fn list_templates(templates_config: &TemplatesConfig) {
    if templates_config.templates.is_empty() {
        println!("No templates found");
        return;
    }

    println!("Templates:");
    for template in &templates_config.templates {
        println!("  {} - {}", template.name, template.description);
    }
}
