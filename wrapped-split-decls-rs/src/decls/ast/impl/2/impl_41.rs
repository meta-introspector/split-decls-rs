use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < AngleBracketedArgs > for Box < GenericArgs > { fn from (val : AngleBracketedArgs) -> Self { Box :: new (GenericArgs :: AngleBracketed (val)) } }