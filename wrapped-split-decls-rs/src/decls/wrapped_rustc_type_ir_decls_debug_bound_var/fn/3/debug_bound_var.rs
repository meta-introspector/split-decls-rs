use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn debug_bound_var < T : std :: fmt :: Write > (fmt : & mut T , bound_index : BoundVarIndexKind , var : impl std :: fmt :: Debug ,) -> Result < () , std :: fmt :: Error > { match bound_index { BoundVarIndexKind :: Bound (debruijn) => { if debruijn == INNERMOST { write ! (fmt , "^{var:?}") } else { write ! (fmt , "^{}_{:?}" , debruijn . index () , var) } } BoundVarIndexKind :: Canonical => write ! (fmt , "^c_{:?}" , var) , } }
}