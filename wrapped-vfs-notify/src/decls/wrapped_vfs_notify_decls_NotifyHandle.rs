use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct NotifyHandle {
    sender: Sender<Message>,
    _thread: stdx::thread::JoinHandle,
}
