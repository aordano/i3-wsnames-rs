use clap;
use config;
use i3ipc::I3Connection as i3;
use i3ipc::I3EventListener as i3event;
use i3ipc::Subscription as event_list;
use lazy_static::lazy_static;
use regex::Regex;
use std::{path, process};

struct I3Data {
    connection: i3,
    events: i3event,
    config: Option<config::Config>,
}

/**
 * TODO
 */
struct ApplyError {}

/**
 * TODO
 */
struct UpdateError {}

#[derive(Debug)]
struct Graph {
    workspaces: Vec<Workspace>,
    windows: Vec<Window>,
    output: String,
}

#[derive(Debug)]
struct Workspace {
    name: String,
    display_name: String,
    id: i64,
    layout: i3ipc::reply::NodeLayout,
    urgent: bool,
    focused: bool,
}

#[derive(Debug)]
struct Window {
    in_workspace: i64,
    i3_id: i64,
    x11_id: i32,
    name: String,
    display_name: String,
    class: String,
    urgent: bool,
    focused: bool,
}

#[derive(Debug)]
enum I3WSNamesError {
    Loop(LoopError),
    Setup(SetupError),
}

#[derive(Debug)]
enum LoopError {
    ApplyError,
    UpdateError,
}

#[derive(Debug)]
enum SetupError {
    I3Connection(i3ipc::EstablishError),
    I3Message(i3ipc::MessageError),
    Config(config::ConfigError),
}

struct WindowDigest {
    window_data: i3ipc::reply::Node,
    workspace_id: i64,
}

struct WorkspaceDigest {
    workspaces: Vec<Workspace>,
    windows: Vec<WindowDigest>,
}

/**
 * Process Windows
 *
 * ---
 *
 * This function gets the digest of windows (the window/container nodes + the id of the workspace they belong to),
 * cleans them up, queries X11 for the window class, and finally returns a clean and nice list of each.
 *  
 * ---
 *
 * TODO Manage display names; load config data and check what they should be and stuff
 *
 */
fn process_windows(windows_digest: Vec<WindowDigest>) -> Vec<Window> {
    let mut windows: Vec<Window> = Vec::new();

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
        let class = WM_CLASS_REGEX
            .captures(&xprop)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        windows.push(Window {
            in_workspace: digest.workspace_id,
            i3_id: digest.window_data.id,
            x11_id: digest.window_data.window.unwrap(),
            name: digest.window_data.name.unwrap(),
            // HACK Load the display name from the config
            display_name: "gotta load this from the config, doug".to_string(),
            urgent: digest.window_data.urgent,
            focused: digest.window_data.focused,
            class: String::from(class),
        })
    }

    windows
}

fn process_workspaces(workspace_nodes: Vec<i3ipc::reply::Node>) -> WorkspaceDigest {
    let mut graph_workspaces = Vec::new();
    let mut window_digest: Vec<WindowDigest> = Vec::new();

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
                        graph_workspaces.push(Workspace {
                            name: name,
                            // HACK Load the display name from the config
                            display_name: "gotta load this from the config, doug".to_string(),
                            id: workspace_id,
                            layout: workspace_layout,
                            urgent: workspace_is_urgent,
                            focused: workspace_is_focused,
                        });
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
                                    window_digest.push(WindowDigest {
                                        window_data: window,
                                        workspace_id: workspace_id,
                                    });
                                }
                            } else {
                                window_digest.push(WindowDigest {
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

    WorkspaceDigest {
        workspaces: graph_workspaces,
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
fn do_initial(data: &mut I3Data) -> Result<Vec<Graph>, I3WSNamesError> {
    let tree = data.connection.get_tree();
    let mut graphs: Vec<Graph> = Vec::new();

    match tree {
        Err(error) => {
            println!("{:?}", error);
            return Err(I3WSNamesError::Setup(SetupError::I3Message(error)));
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
                                                                    process_workspaces(workspaces);

                                                                graphs.push(Graph {
                                                                    workspaces: workspace_digest
                                                                        .workspaces,
                                                                    windows: process_windows(
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
                                            println!("[I3WSNAMES] No output name found");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(graphs)
        }
    }
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
fn do_window(event: i3ipc::event::Event, data: &I3Data) -> Result<(), ApplyError> {
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
fn do_workspace(event: i3ipc::event::Event, data: &I3Data) -> Result<(), ApplyError> {
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
fn move_window(node: i3ipc::reply::Node, data: &I3Data) -> Result<(), ApplyError> {
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
fn rename_workspace(node: i3ipc::reply::Node, data: &I3Data) -> Result<(), ApplyError> {
    // HACK placeholder code
    Ok(())
}

fn main_loop(mut data: I3Data) -> Result<(), LoopError> {
    let workspace = &data.events.subscribe(&[event_list::Workspace]);
    let window = &data.events.subscribe(&[event_list::Window]);
    let mut graph = do_initial(&mut data);
    println!("{:?}", graph);
    let mut event_stream = data.events.listen();

    for event in event_stream {
        match event {
            Ok(this_event) => match this_event {
                i3ipc::event::Event::WorkspaceEvent(workspace_event) => {}
                i3ipc::event::Event::WindowEvent(window_event) => {}
                _ => {}
            },
            Err(error) => {}
        }
    }
    Ok(())
}

fn setup(config_path: Option<&str>) -> Result<I3Data, SetupError> {
    let connection;
    let events;
    let config_data: Option<config::Config>;

    match config_path {
        Some(path) => {
            let config_result = read_config(path);
            match config_result {
                Ok(config) => {
                    config_data = Some(config);
                }
                Err(error) => {
                    println!("{:?}", error);
                    return Err(SetupError::Config(error));
                }
            }
        }
        None => {
            config_data = None;
        }
    }

    match i3::connect() {
        Err(error) => {
            // TODO Fancy error handling here
            println!("{}", error);
            return Err(SetupError::I3Connection(error));
        }
        Ok(conn) => connection = conn,
    }

    match i3event::connect() {
        Err(error) => {
            // TODO Fancy error handling here
            println!("{}", error);
            return Err(SetupError::I3Connection(error));
        }
        Ok(evs) => events = evs,
    }
    Ok(I3Data {
        connection,
        events,
        config: config_data,
    })
}

fn run(matches: clap::ArgMatches) -> Result<(), I3WSNamesError> {
    let config = matches.value_of("config");

    match setup(config) {
        Err(error) => {
            // TODO Fancy error handling here
            match error {
                SetupError::I3Connection(error) => {
                    println!("{}", error);
                    return Err(I3WSNamesError::Setup(SetupError::I3Connection(error)));
                }
                SetupError::I3Message(error) => {
                    println!("{}", error);
                    return Err(I3WSNamesError::Setup(SetupError::I3Message(error)));
                }
                SetupError::Config(error) => {
                    println!("{}", error);
                    return Err(I3WSNamesError::Setup(SetupError::Config(error)));
                }
            }
        }
        Ok(data) => {
            match main_loop(data) {
                Err(error) => {
                    match error {
                        LoopError::ApplyError => {
                            // TODO implement fmt::Display for ApplyError
                            //println!("Something went wrong while applying the changes: {}", error);
                            return Err(I3WSNamesError::Loop(LoopError::ApplyError));
                        }
                        LoopError::UpdateError => {
                            // TODO implement fmt::Display for UpdateError
                            //println!("Something went wrong while updating the workpace graph: {}", error);
                            return Err(I3WSNamesError::Loop(LoopError::UpdateError));
                        }
                    }
                }
                // HACK i mean it should never reach this code unless somthing trips the while breaker
                Ok(_) => {
                    return Ok(());
                }
            }
        }
    }
}

fn main() {
    let matches = clap::Command::new("sitebuilder")
        .version("0.1.0")
        .author("Ágata Ordano")
        .about("Renames workspaces arbitrarily, showing what they contain without losing addressability.AsMut")
        .arg(
            clap::Arg::new("config")
                .takes_value(true)
                .forbid_empty_values(true)
                .short('c')
                .help("Chooses the config file path"),
        )
        .get_matches();
    if let Err(error) = run(matches) {
        println!("Application error: {}", "TODO: Error handling, sorry :(");
        process::exit(1);
    }
}

pub fn read_config(config_path: &str) -> Result<config::Config, config::ConfigError> {
    // Check if a custom path is set and build the PathBuf
    let mut path = path::PathBuf::new();

    path.push(config_path);
    path.push("i3-wsnames");

    // load and return the config
    let config = config::Config::builder()
        .add_source(config::File::new(
            path.to_str().unwrap(),
            config::FileFormat::Json,
        ))
        .build();
    return config;
}
