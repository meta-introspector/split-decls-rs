use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cold] # [inline (never)] fn with_nix_path_allocating < T , F > (from : & [u8] , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { match CString :: new (from) { Ok (s) => Ok (f (& s)) , Err (_) => Err (Errno :: EINVAL) , } }
}