use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl LoggerConfig { pub fn from_env (env : & str) -> Self { LoggerConfig { filter : env :: var (env) , color_logs : env :: var (format ! ("{env}_COLOR")) , verbose_entry_exit : env :: var (format ! ("{env}_ENTRY_EXIT")) , verbose_thread_ids : env :: var (format ! ("{env}_THREAD_IDS")) , backtrace : env :: var (format ! ("{env}_BACKTRACE")) , wraptree : env :: var (format ! ("{env}_WRAPTREE")) , lines : env :: var (format ! ("{env}_LINES")) , } } }
}