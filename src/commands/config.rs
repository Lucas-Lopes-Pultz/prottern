use crate::cli::input::args::Args;
use crate::cli::input::{get, get_valid_input};
use crate::core::config::{self, RemoteRepoRegistry};
use crate::core::path::get_config_path;
use crate::utils::path::pathbuf_to_string;
use crate::utils::errors::invalid_input_error;
use std::io::Error;
use tabled::{Table, Style};

pub fn run(args: Args) -> Result<(), Error> {
    config::create_files()?;

    if args.inputs.len() == 0 {
        return Ok(());
    }

    if args.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_config_path()?));
        return Ok(());
    }

    if args.has_flag("--add") {
        return match args.inputs[0].as_str() {
            "repos:remote" => add_registry_remote_repo(),
            _ => Ok(()),
        };
    }

    if args.has_flag("--remove") {
        return match args.inputs[0].as_str() {
            "repos:remote" => remove_registry_remote_repo(),
            _ => Ok(()),
        };
    }

    if args.has_flag("--update") {
        return match args.inputs[0].as_str() {
            "repos:remote" => update_registry_remote_repo(),
            _ => Ok(()),
        };
    }

    match args.inputs[0].as_str() {
        "repos:remote" => show_registered_remote_repos()?,
        _ => return Err(invalid_input_error("Invalid config option"))
    }

    Ok(())
}

fn show_registered_remote_repos() -> Result<(), Error> {
    let repos = config::repos::remote::get_repos()?;
    let repos_table = Table::new(repos).with(Style::pseudo());

    println!("{}", repos_table);

    Ok(())
}

fn add_registry_remote_repo() -> Result<(), Error> {
    let name = get("Repo name: ")?;
    let url = get("Repo url: ")?;
    let authorization = get_valid_input("Repo requires authorization key? [y/n]: ", None, |input| {
        input == "n" || input == "y" || input == "N" || input == "Y"
    })?;

    let repo_registry = RemoteRepoRegistry {
        name: name.clone(),
        url,
        requires_authorization: authorization == "y" || authorization == "Y"
    };

    config::repos::remote::add(repo_registry.clone())?;

    println!("Repo \"{}\" was registered.", repo_registry.name);

    Ok(())
}

fn remove_registry_remote_repo() -> Result<(), Error> {
    let name = get("Repo name: ")?;
    config::repos::remote::remove(name.clone())?;

    println!("Repo \"{}\" was removed.", name);

    Ok(())
}

fn update_registry_remote_repo() -> Result<(), Error> {
    println!("Press Enter if you want the field remains the same.");

    let current_name = get("Current repo name: ")?;
    let name = get("New repo name: ")?;
    let url = get("New repo url: ")?;
    let authorization = get_valid_input("Repo requires authorization key? [y/n]: ", None, |input| {
        input == "n" || input == "y" || input == "N" || input == "Y"
    })?;

    let repo_updated = RemoteRepoRegistry {
        name: name.clone(),
        url,
        requires_authorization: authorization == "y" || authorization == "Y"
    };

    config::repos::remote::update(&current_name, repo_updated)?;

    println!("Repo \"{}\" was updated.", current_name);

    Ok(())
}