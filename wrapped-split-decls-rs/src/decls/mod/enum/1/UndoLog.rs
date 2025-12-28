use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone)] pub enum UndoLog < K , V > { Inserted (K) , Overwrite (K , V) , Purged , }
}