use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < std :: io :: Error > for GlobError { fn from (e : std :: io :: Error) -> Self { GlobError (e . into ()) } }