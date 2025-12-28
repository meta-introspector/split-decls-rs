use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Defines a single patch file and an optional Git reference for context."] # [derive (Debug , Default , Serialize , Deserialize , Clone)] pub struct PatchSpec { # [doc = " Path to the patch .rs file, relative to the workspace root."] pub path : PathBuf , # [doc = " Optional Git reference (branch, tag, commit hash) associated with this patch."] # [doc = " This indicates the state of the repository for which this patch is relevant."] pub git_reference : Option < String > , }