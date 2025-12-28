use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Credential { # [doc = " Retrieves a token for the given registry."] fn perform (& self , registry : & RegistryInfo < '_ > , action : & Action < '_ > , args : & [& str] ,) -> Result < CredentialResponse , Error > ; }