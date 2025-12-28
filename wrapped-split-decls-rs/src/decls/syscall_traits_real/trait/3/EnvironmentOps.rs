use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Environment operations trait - 2,470 calls (74.2% of all syscalls)"] # [doc = " Strategy: Generic bounds for performance due to high usage"] pub trait EnvironmentOps : Send + Sync { fn get_var (& self , key : & str) -> Option < String > ; fn set_var (& self , key : & str , value : & str) ; fn current_dir (& self) -> IoResult < PathBuf > ; fn args (& self) -> Vec < String > ; fn home_dir (& self) -> Option < PathBuf > ; }
}