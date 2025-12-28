use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: save_cached_matrix");
fn save_cached_matrix (cache_file : & str , matrix : & [Vec < f64 >] , decls : & [Declaration]) -> Result < () > { let cached = CachedMatrix { matrix : matrix . to_vec () , decl_names : decls . iter () . map (| d | d . name . clone ()) . collect () , timestamp : std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) ? . as_secs () , } ; let json = serde_json :: to_string (& cached) ? ; fs :: write (cache_file , json) ? ; println ! ("💾 Matrix cached to {}" , cache_file) ; Ok (()) }
}