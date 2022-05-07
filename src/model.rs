use i3ipc::I3Connection as i3;
use i3ipc::I3EventListener as i3event;

pub struct I3Data {
    pub connection: i3,
    pub events: i3event,
    pub config: Option<config::Config>,
}

/**
 * TODO
 */
pub struct ApplyError {}

/**
 * TODO
 */
pub struct UpdateError {}

#[derive(Debug)]
pub enum I3WSNamesError {
    Loop(LoopError),
    Setup(SetupError),
}

#[derive(Debug)]
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
#[derive(Debug)]
pub struct Tree {
    pub workspaces: Vec<Workspace>,
    pub windows: Vec<Window>,
    pub output: String,
}

#[derive(Debug)]
pub struct Workspace {
    pub name: String,
    pub display_name: String,
    pub id: i64,
    pub layout: i3ipc::reply::NodeLayout,
    pub urgent: bool,
    pub focused: bool,
}

#[derive(Debug)]
pub struct Window {
    pub in_workspace: i64,
    pub i3_id: i64,
    pub x11_id: i32,
    pub name: String,
    pub display_name: String,
    pub class: String,
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
