use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn pat_to_string (ann : & dyn PpAnn , pat : & hir :: Pat < '_ >) -> String { to_string (ann , | s | s . print_pat (pat)) }