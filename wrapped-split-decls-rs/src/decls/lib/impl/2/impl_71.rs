use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialOrd for Span { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& self . data () , & rhs . data ()) } }