use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct PreservationTrace { pub original_package : PackageField , pub toml_reference : Option < String > , pub bootstrap_reference : Option < String > , pub output2_reference : Option < String > , pub preserved : bool , pub transformation_steps : Vec < String > , }