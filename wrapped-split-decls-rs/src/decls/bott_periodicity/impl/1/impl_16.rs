use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Clone for SuspensionTower { fn clone (& self) -> Self { Self { levels : self . levels . clone () , } } }