use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: t5");
# [doc = " Tests zero-length `SourceFile`s."] # [test] fn t5 () { let sm = init_source_map () ; let loc1 = sm . lookup_char_pos (BytePos (22)) ; assert_eq ! (loc1 . file . name , PathBuf :: from ("blork.rs") . into ()) ; assert_eq ! (loc1 . line , 2) ; assert_eq ! (loc1 . col , CharPos (10)) ; let loc2 = sm . lookup_char_pos (BytePos (25)) ; assert_eq ! (loc2 . file . name , PathBuf :: from ("blork2.rs") . into ()) ; assert_eq ! (loc2 . line , 1) ; assert_eq ! (loc2 . col , CharPos (0)) ; }
}