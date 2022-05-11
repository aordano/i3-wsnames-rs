use i3ipc::I3Connection as i3;
use i3ipc::I3EventListener as i3event;
use serde::Deserialize;

#[derive(Debug)]
pub struct I3Data {
    pub connection: i3,
    pub config: Option<Config>,
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
    rename: ConfigRename,
    cross_boundary: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigRename {
    pub windows: Vec<ConfigWindow>,
    pub workspaces: Vec<ConfigWorkspace>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigWindow {
    class: String,
    name: String
}

#[derive(Debug, Deserialize)]
pub struct ConfigWorkspace {
    number: i16,
    name: String
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
    Setup(SetupError),
}

#[derive(Debug, Clone, Copy)]
pub enum LoopError {
    ApplyError,
    UpdateError,
}

#[derive(Debug)]
pub enum SetupError {
    I3Connection(i3ipc::EstablishError),
    I3Message(i3ipc::MessageError),
    Config(config::ConfigError),
}
#[derive(Debug, Clone)]
pub struct Tree {
    pub workspaces: Vec<Workspace>,
    pub windows: Vec<Window>,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String,
    pub display_name: String,
    pub id: i64,
    pub layout: i3ipc::reply::NodeLayout,
    pub urgent: bool,
    pub focused: bool,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub in_workspace: i64,
    pub i3_id: i64,
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
    pub workspaces: Vec<Workspace>,
    pub windows: Vec<WindowDigest>,
}
