use super::model::Config as ConfigStructure;
use clap;
use config;
use i3ipc::event::inner::WorkspaceChange;
use i3ipc::event::WindowEventInfo;
use i3ipc::event::WorkspaceEventInfo;
use i3ipc::reply::CommandOutcome;
use i3ipc::I3Connection;
use std::io::Empty;
use std::{path, process};

use super::model;
use super::tree;

/**
 * TODO
 *
 * This function should move a window to a specific workspace, to match the configuration.
 *
 * TAKES: Event
 * CALLS: move_window
 * RETURNS: corrected graph entry
 */
pub fn do_window(
    event: &WindowEventInfo,
    data: &mut model::I3Data,
    tree: Vec<model::Tree>,
) -> Result<Vec<model::Tree>, model::ApplyError> {
    // HACK placeholder code
    println!("\n[Do Window]\n DEBUG: {:?}", event);
    Ok(tree)
}

/**
 * TODO
 *
 * This function should rename a workspace to match the configuration, and format it accordingly.
 *
 * TAKES: Event
 * CALLS: rename_workspace
 * RETURNS: corrected graph entry
 */
pub fn do_workspace(
    event: &WorkspaceEventInfo,
    data: &mut model::I3Data,
    tree: Vec<model::Tree>,
) -> Result<Vec<model::Tree>, model::I3WSNamesError> {
    // HACK placeholder code
    println!("\n[Do Workspace]\n DEBUG: {:?}", &event);

    match event {
        WorkspaceEventInfo {
            change, current, ..
        } => match change {
            WorkspaceChange::Empty => {
                //First we gotta check if the workspace is already on the tree
                // Hack do some error handling instead of those unwraps
                let current_name = current.as_ref().unwrap().name.as_ref().unwrap();

                for workspace in &data.config.rename.workspaces {
                    if &workspace.number.to_string() == current_name {
                        println!(
                            "\n[Do Workspace]\n DEBUG: New Workspace {:?}, expected {:?}",
                            &current_name,
                            workspace.number.to_string()
                        );

                        match go_to_workspace(String::from(&workspace.name), &mut data.connection) {
                            Ok(()) => {
                                return tree::parse(data);
                            }
                            Err(error) => return Err(error),
                        }
                    }
                }
            }
            // HACK fill the remaining events
            _ => {}
        },
    }

    Ok(tree)
}
fn move_window(
    window_id: i64,
    target_name: String,
    connection: &I3Connection,
) -> Result<(), model::ApplyError> {
    // HACK placeholder code
    Ok(())
}

fn rename_workspace(
    workspace_name: String,
    target_name: String,
    connection: &mut I3Connection,
) -> Result<(), model::I3WSNamesError> {
    let command_line = format!("rename workspace to \"{:}\"", &target_name);

    println!("{}", &command_line);
    return handle_command_reply(connection.run_command(command_line.as_str()));
}

fn go_to_workspace(
    target_name: String,
    connection: &mut I3Connection,
) -> Result<(), model::I3WSNamesError> {
    let command_line = format!("workspace \"{:}\"", &target_name);

    println!("{}", &command_line);

    return handle_command_reply(connection.run_command(command_line.as_str()));
}

fn set_current_layout(
    target_layout: String,
    connection: &mut I3Connection,
) -> Result<(), model::I3WSNamesError> {
    let command_line = format!("layout \"{:}\"", &target_layout);

    println!("{}", &command_line);
    return handle_command_reply(connection.run_command(command_line.as_str()));
}

pub fn handle_command_reply(
    command: Result<i3ipc::reply::Command, i3ipc::MessageError>,
) -> Result<(), model::I3WSNamesError> {
    match command {
        Ok(result) => {
            if result.outcomes[0].success {
                return Ok(());
            }
            return Err(model::I3WSNamesError::Request(
                model::RequestError::I3Command(
                    result.outcomes[0].error.as_ref().unwrap().to_string(),
                ),
            ));
        }
        Err(error) => {
            return Err(model::I3WSNamesError::Request(
                model::RequestError::I3Message(error),
            ))
        }
    };
}

pub fn read_config(config_path: &str) -> Result<model::Config, config::ConfigError> {
    // Check if a custom path is set and build the PathBuf
    let mut path = path::PathBuf::new();

    path.push(config_path);
    //path.push("i3-wsnames");

    // load and return the config
    let config = config::Config::builder()
        .add_source(config::File::new(
            path.to_str().unwrap(),
            config::FileFormat::Json5,
        ))
        .build();

    match config {
        Ok(config_ok) => match config_ok.try_deserialize::<ConfigStructure>() {
            Ok(config_data) => {
                println!("\n[Read Config]\n DEBUG: {:?}", config_data);
                Ok(config_data)
            }
            Err(error) => {
                println!(
                    "\n[Read Config] (Deserialized incorrectly)\n DEBUG: {:?}",
                    &error
                );
                return Err(config::ConfigError::from(error));
            }
        },
        Err(error) => {
            println!("\n[Read Config] (Read incorrectly)\n DEBUG: {:?}", &error);
            return Err(config::ConfigError::from(error));
        }
    }
}
