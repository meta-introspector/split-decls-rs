use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SpanLowerer { fn lower (& self , span : Span) -> Span { if self . is_incremental { span . with_parent (Some (self . def_id)) } else { span } } }