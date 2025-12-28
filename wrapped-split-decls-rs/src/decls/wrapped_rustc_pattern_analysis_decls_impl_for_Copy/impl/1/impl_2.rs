use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'p , Cx : PatCx > Copy for MatchArm < 'p , Cx > { }
}