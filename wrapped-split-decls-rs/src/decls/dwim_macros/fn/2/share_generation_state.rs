use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn share_generation_state (intent : & DwimIntent) -> String { format ! ("https://dwim.split-decls.rs/state/{}" , intent . hash ()) }