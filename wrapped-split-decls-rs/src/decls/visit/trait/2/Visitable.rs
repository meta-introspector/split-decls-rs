use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (crate) trait Visitable < 'a , V : Visitor < 'a > > { type Extra : Copy ; # [must_use] fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result ; }