use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl_from!(Literal < S >, Punct < S >, Ident < S > for Leaf);
