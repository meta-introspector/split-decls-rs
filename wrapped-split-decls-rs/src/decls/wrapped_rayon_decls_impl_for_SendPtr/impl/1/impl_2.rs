use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > SendPtr < T > { fn get (self) -> * mut T { self . 0 } }