use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Nix's main error type."] # [doc = ""] # [doc = " It's a wrapper around Errno.  As such, it's very interoperable with"] # [doc = " [`std::io::Error`], but it has the advantages of:"] # [doc = " * `Clone`"] # [doc = " * `Copy`"] # [doc = " * `Eq`"] # [doc = " * Small size"] # [doc = " * Represents all of the system's errnos, instead of just the most common"] # [doc = "   ones."] pub type Error = Errno ;