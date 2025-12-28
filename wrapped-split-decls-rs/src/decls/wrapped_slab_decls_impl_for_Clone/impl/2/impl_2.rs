use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Clone for Iter < '_ , T > { fn clone (& self) -> Self { Self { entries : self . entries . clone () , len : self . len , } } }