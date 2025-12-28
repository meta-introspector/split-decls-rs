use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < SmolStr > for String { # [inline (always)] fn from (text : SmolStr) -> Self { text . as_str () . into () } }