use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Introspection macro that injects self-awareness at any Bott level"] pub struct IntrospectMacro { current_level : BottLevel , injection_points : Vec < InjectionPoint > , }