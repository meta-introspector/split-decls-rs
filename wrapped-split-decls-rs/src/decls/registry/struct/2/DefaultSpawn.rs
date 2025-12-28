use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Spawns a thread in the \"normal\" way with `std::thread::Builder`."] # [doc = ""] # [doc = " This type is pub-in-private -- E0445 forces us to make it public,"] # [doc = " but we don't actually want to expose these details in the API."] # [derive (Debug , Default)] pub struct DefaultSpawn ;