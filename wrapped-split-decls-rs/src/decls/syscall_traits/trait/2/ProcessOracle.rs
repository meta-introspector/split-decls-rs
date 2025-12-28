use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Process execution oracle  "] pub trait ProcessOracle { fn audit_exec () -> Result < () , String > ; fn check_command_safety (cmd : & str) -> bool ; }