use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The commands are sent by the service control manager to the service through the closure or callback
/// passed to the service `run` method.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    /// The start command is sent when the service first starts.
    Start,
    /// The stop command is sent when the service is stopping just prior to process termination.
    ///
    /// This command will only be sent if the `can_stop` method is called as part of construction.
    Stop,
    /// The pause command is sent when the service is being paused but not stopping.
    ///
    /// This command will only be sent if the `can_pause` method is called as part of construction.
    Pause,
    /// The resume command is sent when the service is being resumed following a pause.
    ///
    /// This command will only be sent if the `can_pause` method is called as part of construction.
    Resume,
    /// An extended command.
    ///
    /// Specific commands will only be received if the `can_accept` methods is called to specify those
    /// commands the service accepts.
    Extended(ExtendedCommand),
}
