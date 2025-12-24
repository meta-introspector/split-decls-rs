use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A command not specifically covered by the `Command` enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExtendedCommand {
    /// The control code for the command.
    pub control: u32,
    /// The event type, if any.
    pub ty: u32,
    /// The event data, if any.
    pub data: *const c_void,
}
