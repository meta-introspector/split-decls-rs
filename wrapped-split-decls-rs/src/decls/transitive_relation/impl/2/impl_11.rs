use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Clone > Clone for TransitiveRelation < T > { fn clone (& self) -> Self { TransitiveRelation { builder : Frozen :: freeze (self . builder . deref () . clone ()) , closure : Frozen :: freeze (self . closure . deref () . clone ()) , } } }