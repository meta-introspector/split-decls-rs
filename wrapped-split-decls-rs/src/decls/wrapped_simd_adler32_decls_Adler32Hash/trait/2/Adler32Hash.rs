use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A Adler-32 hash-able type."] pub trait Adler32Hash { # [doc = " Feeds this value into `Adler32`."] fn hash (& self) -> u32 ; }