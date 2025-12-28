use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < SetGlobalDefaultError > for Error { fn from (tracing_error : SetGlobalDefaultError) -> Self { Error :: AlreadyInit (tracing_error) } }