use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use color_eyre::eyre::{eyre, ContextCompat, Error, Result};
use html_editor::operation::{Editable, Htmlifiable, Selector};
use html_editor::parse;
use log::warn;
use regex::Regex;
use toml_edit::Document;

const MD_PATH: &str = r"c:\Users\User\AppData\Local\Temp\leetcode\editor\en\doc\content\";
const TARGET_PATH: &str = r"c:\Users\User\CLionProjects\leetcode\";

fn main() -> Result<()> {
    pretty_env_logger::init();
    println!("Hello, world!");
    let mut config_file = std::env::current_dir()?;
    config_file.push("cargo.toml");
    let mut projects = Vec::new();
    for file in get_md_files_list() {
        if let Ok(project_name) = create_project_name(&file) {
            let mut file_name = PathBuf::new().join(TARGET_PATH);
            file_name.push(project_name.clone() + r"\README.md");
            if file_name.exists() {
                fs::remove_file(&file)?;
                continue;
            }
            projects.push((project_name, file_name, modified_md(&file)?));
        } else {
            warn!("Failed to create project name for {file:?}");
        }
    }
    if projects.is_empty() {
        return Err(eyre!("No markdown files found"));
    }
    let mut toml = fs::read_to_string(config_file.clone())?.parse::<Document>()?;
    let (_, members) = toml.entry("workspace")
                           .or_insert(toml_edit::table())
                           .as_table_mut()
                           .wrap_err("Failed to get table_mut")?
                           .get_key_value_mut("members")
                           .wrap_err("Failed to find [workspace] members")?;
    let set = members.as_array_mut()
                     .wrap_err("Failed to get as_array_mut")?
                     .iter()
                     .map(|x| x.as_str().unwrap())
                     .collect::<HashSet<_>>();
    let projects = projects.into_iter()
                           .filter(|(project, ..)| !set.contains(project.as_str()))
                           .collect::<Vec<_>>();
    for (project, ..) in projects.iter() {
        members.as_array_mut().wrap_err("Failed to get as_array_mut")?.push(project);
    }
    fs::write(config_file, toml.to_string())?;
    for (project, file_path, content) in projects {
        Command::new("cargo.exe").args(["new", "--lib", project.as_str()]).status()?;
        fs::write(file_path, content)?;
    }
    Ok(())
}

fn modified_md(file: &Path) -> Result<String> {
    let html = fs::read_to_string(file)?;
    let mut document = parse(&html).map_err(Error::msg)?;
    let selector = Selector::from("div, details, summary");
    Ok(document.remove_by(&selector).html())
}

fn create_project_name(from: &Path) -> Result<String> {
    let name = from.file_name()
                   .and_then(|x| x.to_str())
                   .wrap_err(eyre!("Failed to get file name from path: {from:?}"))?;
    let regex = Regex::new(r"\[([[:digit:]]+)\](.+)\.md")?;
    let capt = regex.captures(name)
                    .wrap_err(eyre!("Failed to parse file name with regex: {}", regex.as_str()))?;
    let number = capt.get(1).wrap_err("Failed to get number from file name")?.as_str();
    let remaining = capt.get(2).unwrap().as_str().to_lowercase().replace(' ', "_");
    Ok(format!("s{number}_{remaining}"))
}

fn get_md_files_list() -> Vec<PathBuf> {
    fs::read_dir(MD_PATH).into_iter()
                         .flat_map(|iter| iter.into_iter())
                         .filter_map(Result::ok)
                         .filter(|file| file.file_type().is_ok_and(|f_type| f_type.is_file()))
                         .filter(|file| file.path().extension() == Some("md".as_ref()))
                         .map(|x| x.path())
                         .collect()
}
