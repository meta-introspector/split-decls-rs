use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Filesystem operations trait - 211 calls (6.3% of all syscalls)"] # [doc = " Strategy: Oracle validation for security (path traversal prevention)"] pub trait FileSystemOps : Send + Sync { fn read (& self , path : & Path) -> IoResult < Vec < u8 > > ; fn write (& self , path : & Path , contents : & [u8]) -> IoResult < () > ; fn create_dir_all (& self , path : & Path) -> IoResult < () > ; fn remove_file (& self , path : & Path) -> IoResult < () > ; fn exists (& self , path : & Path) -> bool ; fn metadata (& self , path : & Path) -> IoResult < std :: fs :: Metadata > ; }
}