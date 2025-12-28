use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Spans represent a region of code, used by the IDE to be able link macro inputs and outputs"] # [doc = " together. Positions in spans are relative to some [`SpanAnchor`] to make them more incremental"] # [doc = " friendly."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct SpanData < Ctx > { # [doc = " The text range of this span, relative to the anchor."] # [doc = " We need the anchor for incrementality, as storing absolute ranges will require"] # [doc = " recomputation on every change in a file at all times."] pub range : TextRange , # [doc = " The anchor this span is relative to."] pub anchor : SpanAnchor , # [doc = " The syntax context of the span."] pub ctx : Ctx , }
}