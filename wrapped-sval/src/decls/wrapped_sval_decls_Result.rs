use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/**
A generic streaming result.
*/
pub type Result<T = (), E = Error> = std::result::Result<T, E>;
