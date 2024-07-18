use toml::Table;
use serde_json::from_value;
use std::fs;
use std::io::Read;
use std::process::{exit, Command};
use std::path::Path;
use std::env;
use home::home_dir;

// --- Global Variables ---

// --- Function Definitions ---

// TODO: Add functionality for this feature
fn add_new_lang() -> Result<(), ()> {
    Ok(())
}

fn create_config(path_to_config: &Path) -> std::io::Result<()> {
    let _ = fs::File::create(&path_to_config)?;
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
    let mut commands: Vec<String> = vec![];
    if let Some(sub) = com_table.get(language) {
        let sub_table = sub.as_table();
        if let Some(table) = sub_table {
            for (_key, value) in table.clone() {
                println!("{}", value);
                let tmp = value.as_str().unwrap();
                commands.push(serde_json::from_str(tmp).unwrap());
            }
        }
    }
    return commands;
}

fn execute_commands(proj_dir: &str, name: &String, commands: &Vec<String>) {
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
    let new_project_dir = proj_dir.to_owned() + name;

    // Predefined commands
    let mkdir = Command::new("sh").args(["-c", "mkdir", name]).status().expect("Failed to execute mkdir");
    let cd_touch = Command::new("sh").args(["-c", "cd", name, ";", "touch", "README.md"]).status().expect("Failed to execute cd or touch command");
    let gh_public = Command::new("sh").args(["-c", "gh", "repo", "create", &name, "--public", "--source=."]).status().expect("Failed to create repo");

    println!("made it this far");

    // Current language build commands
    for com in commands {
        let mut build = Command::new("sh");
        build.arg("-c");
        build.arg(com);

        let status = build.status().expect("Failed to build environment");
        assert!(status.success());
    }


    assert!(mkdir.success());
    assert!(cd_touch.success());
    assert!(gh_public.success());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let home = &home_dir().unwrap();
    let default_config = &format!("{}/.config/mkproj/config.toml", home.to_str().unwrap());
    let project_dir = &format!("{}/Projects/", home.to_str().unwrap());


    // Handle the errors of the commands ie. No language provided, invalid lang etc.
    // Standard invocation should be: mkproj <name> <language> <arg1> <arg2> ...

    let mut env_file = fs::File::open(&default_config).expect("FILE FAILED TO OPEN");
    let mut contents = String::new();
    let _ = env_file.read_to_string(&mut contents);

    let supported_langs = contents.parse::<Table>().unwrap();

    let proj_name: &String = &args[1].to_lowercase();
    let proj_lang: &String = &args[2].to_lowercase();
    let proj_args: Vec<String>;
    if args.len() > 3 {
        proj_args = args.get(3..).unwrap().to_vec();
    }

    println!("{}",supported_langs);

    // Check for project language in the supported languages, if it's not there, exit
    if supported_langs.get(proj_lang) == None {
        println!("{} is not a supported language", proj_lang);
        exit(1);
    }

    let build_comms: Vec<String> = parse_commands(&proj_lang, &supported_langs);
    //execute_commands(&project_dir, &proj_name.to_string(), &build_comms);
    println!("Project {} created successfully!", proj_name);
}
