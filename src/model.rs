use i3ipc::I3Connection as i3;
use i3ipc::I3EventListener as i3event;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct I3Data {
    pub connection: i3,
    pub config: Config,
}

#[derive(Debug)]
pub struct SetupData {
    pub data: I3Data,
    pub events: i3event,
}

#[derive(Debug)]
pub struct ConfigData {
    pub rename: ConfigRename,
    pub cross_boundary: bool,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rename: ConfigRename,
    pub cross_boundary: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigRename {
    pub windows: Vec<ConfigWindow>,
    pub workspaces: Vec<ConfigWorkspace>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigWindow {
    pub class: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigWorkspace {
    pub number: u8,
    pub name: String,
    pub layout: Option<String>,
}

/**
 * TODO
 */
#[derive(Debug)]
pub struct ApplyError {}

/**
 * TODO
 */
#[derive(Debug)]
pub struct UpdateError {}

#[derive(Debug)]
pub enum I3WSNamesError {
    Loop(LoopError),
    Request(RequestError),
}

#[derive(Debug, Clone, Copy)]
pub enum LoopError {
    ApplyError,
    UpdateError,
    ParseError,
}

#[derive(Debug)]
pub enum RequestError {
    I3Connection(i3ipc::EstablishError),
    I3Message(i3ipc::MessageError),
    Config(config::ConfigError),
    I3Command(String),
}
#[derive(Debug, Clone)]
pub struct Tree {
    pub workspaces: WorkspaceTree,
    pub windows: WindowTree,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: i64,
    pub layout: i3ipc::reply::NodeLayout,
    pub urgent: bool,
    pub focused: bool,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub in_workspace: i64,
    pub x11_id: i32,
    pub name: String,
    pub display_name: String,
    pub class: Vec<String>,
    pub urgent: bool,
    pub focused: bool,
}

pub struct WindowDigest {
    pub window_data: i3ipc::reply::Node,
    pub workspace_id: i64,
}

pub struct WorkspaceDigest {
    pub workspaces: WorkspaceTree,
    pub windows: Vec<WindowDigest>,
}

// the i64 represents the i3 id given to the entry
pub type WindowTree = BTreeMap<i64, Window>;

// the String represents the workspace name
pub type WorkspaceTree = BTreeMap<String, Workspace>;
