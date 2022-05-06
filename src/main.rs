use clap;
use config;
use i3ipc::I3Connection as i3;
use i3ipc::I3EventListener as i3event;
use i3ipc::Subscription as event_list;
use std::path;

use std::process;

struct I3Data {
    connection: i3,
    events: i3event,
    config: Option<config::Config>,
}

/**
 * TODO
 */
struct Graph {}

/**
 * TODO
 */
struct GraphData {}

/**
 * TODO
 */
struct ApplyError {}

/**
 * TODO
 */
struct UpdateError {}

enum I3WSNamesError {
    Loop(LoopError),
    Setup(SetupError),
}

enum LoopError {
    ApplyError,
    UpdateError,
}

enum SetupError {
    I3Connection(i3ipc::EstablishError),
    I3Message(i3ipc::MessageError),
    Config(config::ConfigError),
}

/**
 * TODO
 *
 * This function should take the new graph and apply it to the current i3 workspace tree.
 *
 * TAKES: graph
 * CALLS: Several i3 Message methods to make the changes
 * RETURNS: Ok(graph) or Err(())
 */
fn apply_graph(graph: &Graph) -> Result<&Graph, ApplyError> {
    // HACK placeholder code
    Ok(graph)
}

/**
 * TODO
 *
 * This function should grab the graph of worspaces and windows, the changes reported by the
 * event listener, and perform whatever correction is necessary to include the changes in the tree.
 *
 * This corrections include relocating windows if they spawn in wrong places, renaming the workspaces
 * to match the configuration, and whatever else is needed (nothing more known so far).
 *
 * TAKES: whole graph, cleaned up i3 IPC data
 * CALLS: relocate_window, rename_workspace
 * RETURNS: OK(graph) or Err(())
 *
 */
fn update(graph: &Graph, data: GraphData) -> Result<&Graph, UpdateError> {
    // HACK placeholder code
    Ok(graph)
}

/**
 * TODO
 *
 * This function should move a window to a specific workspace, to match the configuration.
 *
 * TAKES: graph entry
 * CALLS: none
 * RETURNS: corrected graph entry
 */
fn relocate_window(data: GraphData) -> GraphData {
    // HACK placeholder code
    data
}

/**
 * TODO
 *
 * This function should rename a workspace to match the configuration, and format it accordingly.
 *
 * TAKES: graph entry
 * CALLS: none
 * RETURNS: corrected graph entry
 */
fn rename_workspace(data: GraphData) -> GraphData {
    // HACK placeholder code
    data
}

/**
 *  TODO
 *
 * This function should get the i3 event data and make it useable to the program.
 *
 * To do that, the main focus is to grab changes reported by the listener, and return
 * cleaned interpretations of it, for what's pertinent to the usecase.
 *
 * This means dumping useless data and returning only the info we need.
 *
 * TAKES: raw data from i3 IPC JSON
 * CALLS: none
 * RETURNS: cleaned up data
 *
 * */
fn parse_data(raw_data: i3ipc::event::Event) -> GraphData {
    // HACK placeholder code
    let graph_data: GraphData;
    graph_data
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
fn build_graph() -> Graph {
    // HACK placeholder code
    let graph: Graph;
    graph
}

fn main_loop(mut data: I3Data) -> Result<(), LoopError> {
    let mut running = true; // loop breaker

    let workspace = &data.events.subscribe(&[event_list::Workspace]);
    let window = &data.events.subscribe(&[event_list::Window]);
    let mut event_stream = data.events.listen();
    let mut graph = build_graph();

    while running {
        match event_stream.next() {
            Some(event) => {
                // Remove print statement
                println!("{:?}", event);
                update(&graph, parse_data(event.unwrap())); // HACK remove unwrap
                apply_graph(&graph);
                // TODO: do something to manage errors and stuff
            }
            None => {}
        };
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
