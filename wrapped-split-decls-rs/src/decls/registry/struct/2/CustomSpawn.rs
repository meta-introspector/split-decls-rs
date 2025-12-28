use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Spawns a thread with a user's custom callback."] # [doc = ""] # [doc = " This type is pub-in-private -- E0445 forces us to make it public,"] # [doc = " but we don't actually want to expose these details in the API."] # [derive (Debug)] pub struct CustomSpawn < F > (F) ;
}