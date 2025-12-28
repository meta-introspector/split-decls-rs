use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " IO operations trait - 355 calls (10.6% of all syscalls)  "] # [doc = " Strategy: Dependency injection for testing flexibility"] pub trait IOOps : Send + Sync { fn read_to_string (& self , path : & Path) -> IoResult < String > ; fn write_all (& self , path : & Path , contents : & [u8]) -> IoResult < () > ; fn stdin_read_line (& self , buf : & mut String) -> IoResult < usize > ; fn stdout_write (& self , buf : & [u8]) -> IoResult < () > ; fn stderr_write (& self , buf : & [u8]) -> IoResult < () > ; }