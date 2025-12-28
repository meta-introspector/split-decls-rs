use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " This guard will signal the thread writing the trace file to stop and join it when dropped."] pub struct FlushGuard { sender : Sender < Message > , handle : Cell < Option < JoinHandle < () > > > , }
}