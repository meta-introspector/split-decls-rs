use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A future, stream, sink, or executor that has been instrumented with a `tracing` span."] # [cfg (not (feature = "std-future"))] # [derive (Debug , Clone)] pub struct Instrumented < T > { inner : T , span : Span , }
}