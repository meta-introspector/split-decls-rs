use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : Copy , SM : SpanMapper < S > > SpanMapper < S > for & SM { fn span_for (& self , range : TextRange) -> S { SM :: span_for (self , range) } }