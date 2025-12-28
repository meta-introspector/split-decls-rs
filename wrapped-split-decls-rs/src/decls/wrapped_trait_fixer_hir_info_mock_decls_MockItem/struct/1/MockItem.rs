use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MockItem < 'tcx > (pub Item < 'tcx > , PhantomData < & 'tcx () >) ;