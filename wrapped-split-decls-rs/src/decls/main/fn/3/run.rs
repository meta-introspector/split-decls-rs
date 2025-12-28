use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn run (args : CliArgs) -> Result < () , Box < dyn std :: error :: Error > > { let log_level = if args . debug { LevelFilter :: Debug } else { LevelFilter :: Info } ; env_logger :: Builder :: from_env (Env :: default () . default_filter_or (log_level . to_string ())) . init () ; let duplicates = detect_duplicates (& args , args . threads) ; write_output (& duplicates , & args . output_format . as_str () , args . output_file . as_deref ()) ? ; Ok (()) }