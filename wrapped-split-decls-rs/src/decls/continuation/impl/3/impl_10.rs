use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl DefaultContinuation { pub fn new (state_dir : PathBuf , resolution_dir : PathBuf) -> Self { DefaultContinuation { state_dir , resolution_dir } } fn state_file_path (& self , id : & str) -> PathBuf { self . state_dir . join (format ! ("state_{}.json" , id)) } fn resolution_file_path (& self , id : & str) -> PathBuf { self . resolution_dir . join (format ! ("resolution_{}.json" , id)) } }