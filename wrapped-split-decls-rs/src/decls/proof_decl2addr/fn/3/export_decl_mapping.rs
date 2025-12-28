use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Export declaration mapping as JSON"] fn export_decl_mapping (decls : & HashMap < String , DeclAddress >) -> Result < String > { let mut json = String :: from ("{\n") ; for (i , (name , decl_addr)) in decls . iter () . enumerate () { if i > 0 { json . push_str (",\n") ; } json . push_str (& format ! ("  \"{}\": {{\"addr\": \"{}\", \"type\": \"{}\", \"path\": \"{}\"}}" , name , decl_addr . address , decl_addr . decl_type , decl_addr . source_path)) ; } json . push_str ("\n}") ; Ok (json) }
}