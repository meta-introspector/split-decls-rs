use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Process operations trait - 66 calls (1.9% of all syscalls)"] # [doc = " Strategy: DAO governance required due to security implications"] pub trait ProcessOps : Send + Sync { fn execute (& self , cmd : & str , args : & [& str]) -> IoResult < ProcessOutput > ; fn spawn (& self , cmd : & str) -> IoResult < std :: process :: Child > ; fn current_exe (& self) -> IoResult < PathBuf > ; fn exit (& self , code : i32) -> ! ; }
}