use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The dummy span has zero position, length, and context, and no parent."] pub const DUMMY_SP : Span = Span { lo_or_index : 0 , len_with_tag_or_marker : 0 , ctxt_or_parent_or_marker : 0 } ;