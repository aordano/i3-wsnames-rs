use clap;
use config;
use i3ipc::I3Connection as i3;
use i3ipc::I3EventListener as i3event;
use i3ipc::Subscription as event_list;
use std::{path, process};

mod model;
mod tree;

fn main_loop(mut data: model::I3Data) -> Result<(), model::LoopError> {
    let workspace = &data.events.subscribe(&[event_list::Workspace]);
    let window = &data.events.subscribe(&[event_list::Window]);
    let mut graph = tree::parse(&mut data);
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

fn setup(config_path: Option<&str>) -> Result<model::I3Data, model::SetupError> {
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
                    return Err(model::SetupError::Config(error));
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
            return Err(model::SetupError::I3Connection(error));
        }
        Ok(conn) => connection = conn,
    }

    match i3event::connect() {
        Err(error) => {
            // TODO Fancy error handling here
            println!("{}", error);
            return Err(model::SetupError::I3Connection(error));
        }
        Ok(evs) => events = evs,
    }
    Ok(model::I3Data {
        connection,
        events,
        config: config_data,
    })
}

fn run(matches: clap::ArgMatches) -> Result<(), model::I3WSNamesError> {
    let config = matches.value_of("config");

    match setup(config) {
        Err(error) => {
            // TODO Fancy error handling here
            match error {
                model::SetupError::I3Connection(error) => {
                    println!("{}", error);
                    return Err(model::I3WSNamesError::Setup(
                        model::SetupError::I3Connection(error),
                    ));
                }
                model::SetupError::I3Message(error) => {
                    println!("{}", error);
                    return Err(model::I3WSNamesError::Setup(model::SetupError::I3Message(
                        error,
                    )));
                }
                model::SetupError::Config(error) => {
                    println!("{}", error);
                    return Err(model::I3WSNamesError::Setup(model::SetupError::Config(
                        error,
                    )));
                }
            }
        }
        Ok(data) => {
            match main_loop(data) {
                Err(error) => {
                    match error {
                        model::LoopError::ApplyError => {
                            // TODO implement fmt::Display for ApplyError
                            //println!("Something went wrong while applying the changes: {}", error);
                            return Err(model::I3WSNamesError::Loop(model::LoopError::ApplyError));
                        }
                        model::LoopError::UpdateError => {
                            // TODO implement fmt::Display for UpdateError
                            //println!("Something went wrong while updating the workpace graph: {}", error);
                            return Err(model::I3WSNamesError::Loop(model::LoopError::UpdateError));
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
