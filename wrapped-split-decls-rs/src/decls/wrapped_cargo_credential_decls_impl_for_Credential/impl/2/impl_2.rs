use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Credential for UnsupportedCredential { fn perform (& self , _registry : & RegistryInfo < '_ > , _action : & Action < '_ > , _args : & [& str] ,) -> Result < CredentialResponse , Error > { Err (Error :: UrlNotSupported) } }