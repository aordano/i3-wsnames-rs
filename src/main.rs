use clap;
use config;
use i3ipc::I3Connection as i3;
use i3ipc::I3EventListener as i3event;
use i3ipc::Subscription as event_list;
use std::{path, process};

mod functions;
mod model;
mod tree;

fn main() {
    let matches = clap::Command::new("i3-wsnames")
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
fn run(matches: clap::ArgMatches) -> Result<(), model::I3WSNamesError> {
    let config = matches.value_of("config");

    match setup(config) {
        Err(error) => {
            // TODO Fancy error handling here
            match error {
                model::RequestError::I3Connection(error) => {
                    println!("\n DEBUG: {}", error);
                    return Err(model::I3WSNamesError::Request(
                        model::RequestError::I3Connection(error),
                    ));
                }
                model::RequestError::I3Message(error) => {
                    println!("\n DEBUG: {}", error);
                    return Err(model::I3WSNamesError::Request(
                        model::RequestError::I3Message(error),
                    ));
                }
                model::RequestError::Config(error) => {
                    println!("\n DEBUG: {}", error);
                    return Err(model::I3WSNamesError::Request(model::RequestError::Config(
                        error,
                    )));
                }
                model::RequestError::I3Command(error) => {
                    println!("\n DEBUG: {}", error);
                    return Err(model::I3WSNamesError::Request(
                        model::RequestError::I3Command(error),
                    ));
                }
            }
        }
        Ok(setup_data) => {
            match main_loop(setup_data.data, setup_data.events) {
                Err(error) => return Err(error),
                // HACK i mean it should never reach this code unless somthing trips the while breaker
                Ok(_) => {
                    return Ok(());
                }
            }
        }
    }
}
fn setup(config_path: Option<&str>) -> Result<model::SetupData, model::RequestError> {
    let connection;
    let events;
    let config_data: model::Config;

    let default_config = model::Config {
        rename: model::ConfigRename {
            workspaces: Vec::<model::ConfigWorkspace>::new(),
            windows: Vec::<model::ConfigWindow>::new(),
        },
        cross_boundary: false,
    };

    match config_path {
        Some(path) => {
            let config_result = functions::read_config(path);
            match config_result {
                Ok(config) => {
                    config_data = config;
                }
                Err(error) => {
                    println!("\n DEBUG: {:?}", error);
                    return Err(model::RequestError::Config(error));
                }
            }
        }
        None => {
            config_data = default_config;
        }
    }

    match i3::connect() {
        Err(error) => {
            // TODO Fancy error handling here
            println!("\n DEBUG: {}", error);
            return Err(model::RequestError::I3Connection(error));
        }
        Ok(conn) => connection = conn,
    }

    match i3event::connect() {
        Err(error) => {
            // TODO Fancy error handling here
            println!("\n DEBUG: {}", error);
            return Err(model::RequestError::I3Connection(error));
        }
        Ok(evs) => events = evs,
    }
    Ok(model::SetupData {
        data: model::I3Data {
            connection,
            config: config_data,
        },
        events: events,
    })
}

fn main_loop(mut data: model::I3Data, mut events: i3event) -> Result<(), model::I3WSNamesError> {
    let workspace = &events.subscribe(&[event_list::Workspace]);
    let window = &events.subscribe(&[event_list::Window]);
    let mut event_stream = events.listen();

    let mut tree: Vec<model::Tree>;

    match tree::parse(&mut data) {
        Ok(ok_tree) => {
            tree = ok_tree;
            println!("\n[Parse Tree]\n DEBUG: {:?}", &tree);
        }
        Err(error) => {
            println!("\n DEBUG: {:?}", error);
            return Err(error);
        }
    }

    for event in event_stream {
        match event {
            Ok(this_event) => match this_event {
                i3ipc::event::Event::WorkspaceEvent(workspace_event) => {
                    tree = functions::do_workspace(&workspace_event, &mut data, tree).unwrap();
                }
                i3ipc::event::Event::WindowEvent(window_event) => {
                    tree = functions::do_window(&window_event, &mut data, tree).unwrap();
                }
                _ => {}
            },
            Err(error) => {}
        }
    }
    Ok(())
}
