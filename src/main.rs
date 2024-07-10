use toml::Table;
use std::{fs, io};
use std::process::{exit, Command};
use std::path::PathBuf;
use std::env;

// --- Global Variables ---

// --- Function Definitions ---

// TODO: Add functionality for this feature
fn add_new_lang() -> Result<(), io::Error> {
    Ok(())
}

fn config_exists(path_to_config: &PathBuf) -> bool {
    return fs::metadata(path_to_config).is_ok();
}

fn create_config(path_to_config: &PathBuf) -> std::io::Result<()> {
    let _ = fs::File::create(path_to_config)?;
    Ok(())
}

fn parse_commands(language: &String, com_table: &Table) -> Vec<String> {
    /*
        Function to get the commands from the table, and return only the commands pertaining to that language
        -- Parameters --
        language : &String
            - the main language you'd use for the project

        com_table : &Table
            - A parsed (key, value) version of the TOML file that contains the commands to build the env

        -- Returns --
        Vec<String> : Contains the relevant commands to be executed

     */
    let sub_table = com_table[language].as_table().unwrap();
    let mut commands: Vec<String> = vec![];

    for (label, _command) in sub_table {
        commands.push(com_table[label].to_string());
    }

    return commands;
}

fn execute_commands(name: String, commands: &Vec<String>) {
    /*
        Function to organize and excecute the various commands needed to build the project
        -- Parameters --
        name : String
            - Name of the project

        commands : &Vec<String>
            - Relevant commands to be executed to build the project

        -- Returns --
        None
     */

    // Predefined commands
    let mkdir = Command::new("sh").args(["-c", "mkdir", &name]).status().expect("Failed to execute mkdir");
    let cd_touch = Command::new("sh").args(["-c", "cd", &name, ";", "touch", "README.md"]).status().expect("Failed to execute cd or touch command");
    let gh_public = Command::new("sh").args(["-c", "gh", "repo", "create", &name, "--public", "--source=."]).status().expect("Failed to create repo");

    // Current language build commands
    let mut build = Command::new("sh");
    build.arg("-c");
    for com in commands {
        build.arg(com);
    }

    let status = build.status().expect("Failed to build environment");

    assert!(status.success());
    assert!(mkdir.success());
    assert!(cd_touch.success());
    assert!(gh_public.success());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_config: PathBuf = PathBuf::from("~/.config/mkproj/config.toml");

    if !config_exists(&default_config) {
        create_config(&default_config).unwrap();
    }

    // Handle the errors of the commands ie. No language provided, invalid lang etc.
    // Standard invocation should be: mkproj <name> <language> <arg1> <arg2> ...

    let env_file = fs::read_to_string(default_config).unwrap();

    let supported_langs: Table = env_file.parse().unwrap();

    let proj_name: &String = &args[1].to_lowercase();
    let proj_lang: &String = &args[2].to_lowercase();
    let proj_args: Vec<String>;
    if args.len() > 2 {
        proj_args = args.get(3..).unwrap().to_vec();
    }

    // Check for project language in the supported languages, if it's not there, exit
    if !supported_langs.contains_key(proj_lang) {
        println!("{} is not a supported language", proj_lang);
        exit(1);
    }

    let build_comms: Vec<String> = parse_commands(proj_lang, &supported_langs);
    execute_commands(proj_name.to_string(), &build_comms);
    println!("Project {} created successfully!", proj_name);
}
