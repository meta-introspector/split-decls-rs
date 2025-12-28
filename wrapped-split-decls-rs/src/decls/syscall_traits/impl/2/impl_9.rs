use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ProcessOracle for DefaultProcessOracle { fn audit_exec () -> Result < () , String > { eprintln ! ("PROC_AUDIT: Exec operation") ; Ok (()) } fn check_command_safety (cmd : & str) -> bool { ! cmd . contains ("rm -rf") && ! cmd . contains ("sudo") } }