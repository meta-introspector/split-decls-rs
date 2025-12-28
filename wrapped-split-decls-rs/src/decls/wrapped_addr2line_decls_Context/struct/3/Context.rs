use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The state necessary to perform address to line translation."] # [doc = ""] # [doc = " Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s"] # [doc = " when performing lookups for many addresses in the same executable."] pub struct Context < R : gimli :: Reader > { sections : Arc < gimli :: Dwarf < R > > , units : ResUnits < R > , sup_units : SupUnits < R > , }