use clap;
use config;
use i3ipc::I3Connection as i3;
use i3ipc::I3EventListener as i3event;
use i3ipc::Subscription as event_list;
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
    event: i3ipc::event::Event,
    data: &model::I3Data,
) -> Result<(), model::ApplyError> {
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
pub fn do_workspace(
    event: i3ipc::event::Event,
    data: &model::I3Data,
) -> Result<(), model::ApplyError> {
    // HACK placeholder code
    Ok(())
}

/**
 * TODO
 *
 * This function should move a window to a specific workspace, to match the configuration.
 *
 * TAKES: Event
 * CALLS: move_window
 * RETURNS: corrected graph entry
 */
pub fn move_window(
    node: i3ipc::reply::Node,
    data: &model::I3Data,
) -> Result<(), model::ApplyError> {
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
pub fn rename_workspace(
    node: i3ipc::reply::Node,
    data: &model::I3Data,
) -> Result<(), model::ApplyError> {
    // HACK placeholder code
    Ok(())
}

/**
 * TODO
 *
 * This function should get an event and perform the corresponding action to fix the graph.
 *
 * TAKES: Event, I3Data, Tree
 * CALLS: do_window, do_workspace
 * RETURNS: corrected graph entry
 */
pub fn handle_event(
    event: i3ipc::event::Event,
    data: &model::I3Data,
    tree: model::Tree,
) -> Result<model::Tree, model::ApplyError> {
    // HACK placeholder code
    Ok(())
}
