use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Automatic tag following options."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum AutotagOption { # [doc = " Use the setting from the remote's configuration"] Unspecified , # [doc = " Ask the server for tags pointing to objects we're already downloading"] Auto , # [doc = " Don't ask for any tags beyond the refspecs"] None , # [doc = " Ask for all the tags"] All , }
}