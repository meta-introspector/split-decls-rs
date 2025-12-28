use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Serialize , Deserialize , Clone , Debug , PartialEq , Eq)] # [serde (rename_all = "kebab-case")] pub struct LoginOptions < 'a > { # [doc = " Token passed on the command line via --token or from stdin"] # [serde (skip_serializing_if = "Option::is_none")] pub token : Option < Secret < & 'a str > > , # [doc = " Optional URL that the user can visit to log in to the registry"] # [serde (skip_serializing_if = "Option::is_none")] pub login_url : Option < & 'a str > , }