use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FlushGuard { # [doc = " Signals the trace writing thread to flush to disk."] pub fn flush (& self) { if let Some (handle) = self . handle . take () { let _ignored = self . sender . send (Message :: Flush) ; self . handle . set (Some (handle)) ; } } # [doc = " Finishes the current trace and starts a new one."] # [doc = ""] # [doc = " If a [`Write`](std::io::Write) implementation is supplied,"] # [doc = " the new trace is written to it. Otherwise, the new trace"] # [doc = " goes to `./trace-{unix epoc in micros}.json`."] pub fn start_new (& self , writer : Option < Box < dyn Write + Send > >) { if let Some (handle) = self . handle . take () { let _ignored = self . sender . send (Message :: StartNew (writer)) ; self . handle . set (Some (handle)) ; } } }
}