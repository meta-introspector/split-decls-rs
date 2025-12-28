use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Serialize , Deserialize , Clone , Debug , PartialEq , Eq)] # [non_exhaustive] # [serde (tag = "kind" , rename_all = "kebab-case")] pub enum Action < 'a > { # [serde (borrow)] Get (Operation < 'a >) , Login (LoginOptions < 'a >) , Logout , # [serde (other)] Unknown , }