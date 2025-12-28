use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl EventArgRecorder < '_ > { # [doc = " Records a single argument within the current generic activity being profiled."] # [doc = ""] # [doc = " Note: when self-profiling with costly event arguments, at least one argument"] # [doc = " needs to be recorded. A panic will be triggered if that doesn't happen."] pub fn record_arg < A > (& mut self , event_arg : A) where A : Borrow < str > + Into < String > , { let event_arg = self . profiler . get_or_alloc_cached_string (event_arg) ; self . args . push (event_arg) ; } }
}