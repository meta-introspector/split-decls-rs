use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Internable + ? Sized > Clone for Interned < T > { fn clone (& self) -> Self { Self { arc : self . arc . clone () , } } }