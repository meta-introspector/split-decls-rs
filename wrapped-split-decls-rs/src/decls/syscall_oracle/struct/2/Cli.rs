use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "syscall-oracle")] # [command (about = "Syscall Oracle - Type-safe syscall interception with DAO governance")] struct Cli { # [command (subcommand)] command : Commands , }