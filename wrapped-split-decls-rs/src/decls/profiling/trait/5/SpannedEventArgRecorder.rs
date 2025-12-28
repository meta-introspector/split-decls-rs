use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Extension trait for self-profiling purposes: allows to record spans within a generic activity's"] # [doc = " event arguments."] pub trait SpannedEventArgRecorder { # [doc = " Records the following event arguments within the current generic activity being profiled:"] # [doc = " - the provided `event_arg`"] # [doc = " - a string representation of the provided `span`"] # [doc = ""] # [doc = " Note: when self-profiling with costly event arguments, at least one argument"] # [doc = " needs to be recorded. A panic will be triggered if that doesn't happen."] fn record_arg_with_span < A > (& mut self , source_map : & SourceMap , event_arg : A , span : crate :: Span) where A : Borrow < str > + Into < String > ; }
}