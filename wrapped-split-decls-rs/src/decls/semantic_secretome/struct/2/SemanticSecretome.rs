use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct SemanticSecretome { functions : HashMap < String , String > , types : HashMap < String , String > , constants : HashMap < String , String > , modules : HashMap < String , String > , total_symbols : usize , semantic_groups : HashMap < String , Vec < String > > , }