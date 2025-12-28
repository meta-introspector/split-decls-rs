use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn run_ignored (test : & str) -> ExitStatus { Command :: new (env :: current_exe () . unwrap ()) . arg ("--ignored") . arg ("--exact") . arg (test) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () . unwrap () }