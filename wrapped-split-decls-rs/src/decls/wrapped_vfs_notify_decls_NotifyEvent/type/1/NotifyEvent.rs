use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type NotifyEvent = notify :: Result < notify :: Event > ;
}