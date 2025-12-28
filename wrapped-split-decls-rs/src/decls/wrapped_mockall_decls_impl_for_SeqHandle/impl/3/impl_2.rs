use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SeqHandle { # [doc = " Tell the Sequence that this expectation has been fully satisfied"] pub fn satisfy (& self) { self . inner . satisfy (self . seq) ; } # [doc = " Verify that this handle was called in the correct order"] pub fn verify < F : Fn () -> String > (& self , desc : F) { self . inner . verify (self . seq , desc) ; } }