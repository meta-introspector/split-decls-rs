use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 's > Iterator for ShortFlags < 's > { type Item = Result < char , & 's OsStr > ; fn next (& mut self) -> Option < Self :: Item > { self . next_flag () } }