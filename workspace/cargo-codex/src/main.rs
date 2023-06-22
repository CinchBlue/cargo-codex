use cargo_codex::cli::Args;
use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::Parser;
use std::error::Error;

fn get_transitive_dependencies(crate_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut cmd = MetadataCommand::new();
    cmd.no_deps().manifest_path("./Cargo.toml");
    let metadata = cmd.exec()?;
    
    let package = metadata
        .packages
        .iter()
        .find(|p| p.name == crate_name)
        .ok_or_else(|| format!("No package found with name: {}", crate_name))?;
    
    let mut dependencies = Vec::new();
    
    for dep in &package.dependencies {
        if !dependencies.contains(&dep.name) {
            dependencies.push(dep.name.clone());
        }
    }
    
    Ok(dependencies)
}

fn main() -> eyre::Result<()> {
    let args: Args= cargo_codex::cli::Args::parse();
    let crate_name = args.name;

    
    
    match get_transitive_dependencies(&crate_name) {
        Ok(dependencies) => {
            println!("Transitive dependencies of '{}':", crate_name);
            for dependency in dependencies {
                println!("{}", dependency);
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
    Ok(())
}