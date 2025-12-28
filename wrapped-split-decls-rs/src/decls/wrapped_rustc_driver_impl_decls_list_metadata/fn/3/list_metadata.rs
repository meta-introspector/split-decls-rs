use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn list_metadata (sess : & Session , metadata_loader : & dyn MetadataLoader) { match sess . io . input { Input :: File (ref path) => { let mut v = Vec :: new () ; locator :: list_file_metadata (& sess . target , path , metadata_loader , & mut v , & sess . opts . unstable_opts . ls , sess . cfg_version ,) . unwrap () ; safe_println ! ("{}" , String :: from_utf8 (v) . unwrap ()) ; } Input :: Str { .. } => { # [allow (rustc :: diagnostic_outside_of_impl)] sess . dcx () . fatal ("cannot list metadata for stdin") ; } } }