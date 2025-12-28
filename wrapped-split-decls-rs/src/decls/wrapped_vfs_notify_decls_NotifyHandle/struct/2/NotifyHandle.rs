use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct NotifyHandle { sender : Sender < Message > , _thread : stdx :: thread :: JoinHandle , }
}