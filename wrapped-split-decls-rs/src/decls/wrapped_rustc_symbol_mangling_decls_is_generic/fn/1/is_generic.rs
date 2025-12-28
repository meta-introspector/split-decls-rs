use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn is_generic < 'tcx > (instance : Instance < 'tcx >) -> bool { instance . args . non_erasable_generics () . next () . is_some () }