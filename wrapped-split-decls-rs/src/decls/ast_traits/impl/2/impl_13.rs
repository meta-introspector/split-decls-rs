use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : HasTokens > HasTokens for Option < T > { fn tokens (& self) -> Option < & LazyAttrTokenStream > { self . as_ref () . and_then (| inner | inner . tokens ()) } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { self . as_mut () . and_then (| inner | inner . tokens_mut ()) } }