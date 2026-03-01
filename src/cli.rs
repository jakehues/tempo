use clap::{Parser, Subcommand};

use crate::{
    config::{self, get_templates_dir},
    error, manifest,
    template::{create_new_template, create_project, delete_template, list_templates},
};

pub const APP_NAME: &str = "tempo";
pub const APP_ABOUT: &str = "A command-line code templating tool";

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(version, about = APP_ABOUT, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: TopLevelCommands,
}

#[derive(Subcommand)]
enum TopLevelCommands {
    /// Create a new project
    New { name: String },

    /// List all templates
    List,

    /// Template management commands
    Template {
        #[command(subcommand)]
        command: TemplateCommands,
    },
}

#[derive(Subcommand, Clone)]
enum TemplateCommands {
    /// Create a new template
    New { name: String },

    /// Initialize a new template manifest
    Init,

    /// Delete a template
    Delete { name: String },
}

pub fn run_cli() -> error::CliResult<()> {
    // Ensure config directories and files exists
    config::ensure_templates_file(APP_NAME)?;
    config::ensure_template_dir(APP_NAME)?;
    let mut templates = config::load_templates_file(APP_NAME)?;
    let templates_dir = get_templates_dir(APP_NAME)?;

    let cli = Cli::parse();

    match cli.command {
        TopLevelCommands::New { name } => {
            create_project(&name, &templates, &templates_dir)?;
            println!("Created new project from template: {}", name);
        }
        TopLevelCommands::List => {
            list_templates(&templates);
        }

        TopLevelCommands::Template { command } => match command {
            TemplateCommands::New { name } => {
                create_new_template(
                    &name,
                    &"tempo.toml",
                    &mut templates,
                    &templates_dir,
                    APP_NAME,
                )?;
                println!("Created new template: {}", name);
            }
            TemplateCommands::Init => {
                manifest::init_manifest(&"tempo.toml")?;
                println!("Successfully initialized manifest!");
            }
            TemplateCommands::Delete { name } => {
                delete_template(&name, &mut templates, &templates_dir, APP_NAME)?;
                println!("Deleted template: {}", name);
            }
        },
    }

    Ok(())
}
