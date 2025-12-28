use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum GenericBound { Trait (PolyTraitRef) , Outlives (# [visitable (extra = LifetimeCtxt :: Bound)] Lifetime) , # [doc = " Precise capturing syntax: `impl Sized + use<'a>`"] Use (ThinVec < PreciseCapturingArg > , Span) , }