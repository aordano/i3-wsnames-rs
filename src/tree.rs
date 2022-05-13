use i3ipc;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::process;

use super::model;

/// Process Windows
///
/// ---
///
/// This function gets the digest of windows (the window/container nodes + the id of the workspace they
/// belong to), cleans them up, queries X11 for the window class, and finally returns a
/// clean and nice list of each.
///
/// ---
///
/// TODO Manage display names; load config data and check what they should be and stuff
///
///
pub fn parse_windows(windows_digest: Vec<model::WindowDigest>) -> model::WindowTree {
    let mut windows: model::WindowTree = BTreeMap::new();

    // Defines constant regex to only select the actual class name
    lazy_static! {
        static ref WM_CLASS_REGEX: Regex = Regex::new(r#""([^"]+)""#).unwrap();
    }

    for digest in windows_digest {
        // Calls xprop to request the class name of the window
        let xprop = String::from_utf8(
            process::Command::new("xprop")
                .arg("-id")
                .arg(digest.window_data.window.unwrap().to_string())
                .arg("WM_CLASS")
                .output()
                .expect("[XPROP] ")
                .stdout,
        )
        .unwrap();

        // Matches the first class name (usually further names are minor variations of the first)
        let class_matches = WM_CLASS_REGEX.captures_iter(&xprop);

        let mut classes = Vec::new();

        for capture in class_matches {
            classes.push(capture.get(1).unwrap().as_str().to_string());
        }

        windows.insert(
            digest.window_data.id,
            model::Window {
                in_workspace: digest.workspace_id,
                x11_id: digest.window_data.window.unwrap(),
                name: digest.window_data.name.unwrap(),
                // HACK Load the display name from the config
                display_name: "gotta load this from the config, doug".to_string(),
                urgent: digest.window_data.urgent,
                focused: digest.window_data.focused,
                class: classes,
            },
        );
    }

    windows
}

pub fn parse_workspaces(workspace_nodes: Vec<i3ipc::reply::Node>) -> model::WorkspaceDigest {
    let mut tree_workspaces = BTreeMap::new();
    let mut window_digest: Vec<model::WindowDigest> = Vec::new();

    for workspace in workspace_nodes {
        match workspace {
            i3ipc::reply::Node {
                nodes: containers,
                name: workspace_name,
                id: workspace_id,

                layout: workspace_layout,
                urgent: workspace_is_urgent,
                focused: workspace_is_focused,
                ..
            } => {
                match workspace_name {
                    Some(name) => {
                        tree_workspaces.insert(
                            name,
                            model::Workspace {
                                id: workspace_id,
                                layout: workspace_layout,
                                urgent: workspace_is_urgent,
                                focused: workspace_is_focused,
                            },
                        );
                    }
                    None => {
                        println!("[I3WSNAMES] No workspace name found");
                    }
                }
                for container in containers {
                    match &container {
                        i3ipc::reply::Node { nodes: windows, .. } => {
                            if windows.len() > 0 {
                                for window in windows.clone() {
                                    window_digest.push(model::WindowDigest {
                                        window_data: window,
                                        workspace_id: workspace_id,
                                    });
                                }
                            } else {
                                window_digest.push(model::WindowDigest {
                                    window_data: container,
                                    workspace_id: workspace_id,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    model::WorkspaceDigest {
        workspaces: tree_workspaces,
        windows: window_digest,
    }
}

/**
 * TODO
 *
 * This function should query the i3 for the current workspace names and structure.
 *
 * With this info, make a small graph that can represent the structure with some metadata to
 * handle the changes.
 *
 * This way, when something changes, it's easy to check what it is and where it should go.
 *
 * TAKES: Nothing
 * CALLS: Several i3 Message methods to perform the query
 * RETURNS: graph containing the state of the i3 workspace tree
 */
pub fn parse(data: &mut model::I3Data) -> Result<Vec<model::Tree>, model::I3WSNamesError> {
    let tree_per_output = data.connection.get_tree();
    let mut trees: Vec<model::Tree> = Vec::new();

    match tree_per_output {
        Err(error) => {
            println!("{:?}", error);
            return Err(model::I3WSNamesError::Request(
                model::RequestError::I3Message(error),
            ));
        }
        Ok(tree) => {
            match tree {
                i3ipc::reply::Node { nodes, .. } => {
                    for node in nodes {
                        match node {
                            i3ipc::reply::Node {
                                nodes: outputs,
                                name: output_name,
                                nodetype,
                                ..
                            } => {
                                if nodetype == i3ipc::reply::NodeType::Output {
                                    match output_name {
                                        Some(name) => {
                                            if &name != &String::from("__i3") {
                                                for area in outputs {
                                                    match area {
                                                        i3ipc::reply::Node {
                                                            nodes: workspaces,
                                                            nodetype: area_type,
                                                            ..
                                                        } => {
                                                            if area_type
                                                                == i3ipc::reply::NodeType::Con
                                                            {
                                                                let workspace_digest =
                                                                    parse_workspaces(workspaces);

                                                                trees.push(model::Tree {
                                                                    workspaces: workspace_digest
                                                                        .workspaces,
                                                                    windows: parse_windows(
                                                                        workspace_digest.windows,
                                                                    ),
                                                                    output: name.clone(),
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        None => {
                                            println!("[I3WSNAMES] No output found");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(trees)
        }
    }
}
