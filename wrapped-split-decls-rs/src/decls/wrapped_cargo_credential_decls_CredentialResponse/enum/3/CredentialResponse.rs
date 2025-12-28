use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Message sent by the credential helper"] # [derive (Serialize , Deserialize , Clone , Debug , PartialEq , Eq)] # [serde (tag = "kind" , rename_all = "kebab-case")] # [non_exhaustive] pub enum CredentialResponse { Get { token : Secret < String > , # [serde (flatten)] cache : CacheControl , operation_independent : bool , } , Login , Logout , # [serde (other)] Unknown , }
}