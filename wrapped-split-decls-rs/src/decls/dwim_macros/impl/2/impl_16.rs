use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl syn :: parse :: Parse for DwimIntent { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { Ok (DwimIntent { keywords : vec ! ["bootstrap" . to_string ()] , context : "default" . to_string () , }) } }