use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Changed file in the [`Vfs`]."] # [derive (Debug)] pub struct ChangedFile { # [doc = " Id of the changed file"] pub file_id : FileId , # [doc = " Kind of change"] pub change : Change , }