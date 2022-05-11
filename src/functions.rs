use super::model::Config as ConfigStructure;
use clap;
use config;
use i3ipc::event::WindowEventInfo;
use i3ipc::event::WorkspaceEventInfo;
use std::{path, process};

use super::model;

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
    tree: model::Tree,
) -> Result<model::Tree, model::ApplyError> {
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
    tree: model::Tree,
) -> Result<model::Tree, model::ApplyError> {
    // HACK placeholder code
    println!("\n[Do Workspace]\n DEBUG: {:?}", event);
    Ok(tree)
}

fn locate() {}

/**
 * TODO
 *
 * This function should move a window to a specific workspace, to match the configuration.
 *
 * TAKES: Event
 * CALLS: move_window
 * RETURNS: corrected graph entry
 */
fn move_window(node: i3ipc::reply::Node, data: &model::I3Data) -> Result<(), model::ApplyError> {
    // HACK placeholder code
    Ok(())
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
fn rename_workspace(
    node: i3ipc::reply::Node,
    data: &model::I3Data,
) -> Result<(), model::ApplyError> {
    // HACK placeholder code
    Ok(())
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
        Ok(config_ok) => {
            //println!("\n[Read Config]\n DEBUG: {:?}", config_ok);
            match config_ok.try_deserialize::<ConfigStructure>() {
                Ok(config_data) => {
                    println!("\n[Read Config]\n DEBUG: {:?}", config_data);
                    Ok(config_data)
                },
                Err(error) => {
                    println!("\n[Read Config] (Deserialized incorrectly)\n DEBUG: {:?}", &error);
                    return Err(config::ConfigError::from(error));
                }
            }
        }
        Err(error) => {
            println!("\n[Read Config] (Read incorrectly)\n DEBUG: {:?}", &error);
            return Err(config::ConfigError::from(error));
        }
    }
}
