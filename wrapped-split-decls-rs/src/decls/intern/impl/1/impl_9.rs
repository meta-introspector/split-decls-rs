use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T > Clone for Interned < 'a , T > { fn clone (& self) -> Self { * self } }