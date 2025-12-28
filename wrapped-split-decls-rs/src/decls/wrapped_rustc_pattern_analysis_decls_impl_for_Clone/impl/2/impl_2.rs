use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'p , Cx : PatCx > Clone for MatchArm < 'p , Cx > { fn clone (& self) -> Self { Self { pat : self . pat , has_guard : self . has_guard , arm_data : self . arm_data , } } }