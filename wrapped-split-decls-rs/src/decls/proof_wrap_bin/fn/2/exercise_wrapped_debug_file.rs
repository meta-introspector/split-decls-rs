use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Exercise wrapped DebugFile enum (simulated)  "] fn exercise_wrapped_debug_file () -> String { # [derive (Debug)] enum DebugFile { Primary , Supplementary , Dwo } let file = DebugFile :: Primary ; format ! ("DebugFile::{:?} variant used successfully" , file) }
}