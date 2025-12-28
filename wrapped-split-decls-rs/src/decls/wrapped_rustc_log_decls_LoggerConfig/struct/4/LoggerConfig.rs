use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The values of all the environment variables that matter for configuring a logger."] # [doc = " Errors are explicitly preserved so that we can share error handling."] pub struct LoggerConfig { pub filter : Result < String , VarError > , pub color_logs : Result < String , VarError > , pub verbose_entry_exit : Result < String , VarError > , pub verbose_thread_ids : Result < String , VarError > , pub backtrace : Result < String , VarError > , pub wraptree : Result < String , VarError > , pub lines : Result < String , VarError > , }
}