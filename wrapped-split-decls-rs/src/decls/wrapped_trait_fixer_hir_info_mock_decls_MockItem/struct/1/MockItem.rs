use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct MockItem < 'tcx > (pub Item < 'tcx > , PhantomData < & 'tcx () >) ;
}