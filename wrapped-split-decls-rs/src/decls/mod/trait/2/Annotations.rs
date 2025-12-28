use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An accumulator for annotations."] pub trait Annotations < N : Idx > { type Ann : Annotation ; type SccIdx : Idx + Ord ; fn new (& self , element : N) -> Self :: Ann ; fn annotate_scc (& mut self , scc : Self :: SccIdx , annotation : Self :: Ann) ; }