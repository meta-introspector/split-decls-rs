use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Credential provider that doesn't support any registries."] pub struct UnsupportedCredential ;
}