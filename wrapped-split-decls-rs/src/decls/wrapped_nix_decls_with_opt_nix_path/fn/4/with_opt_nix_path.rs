use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Like `NixPath::with_nix_path()`, but allow the `path` argument to be optional."] # [doc = ""] # [doc = " A NULL pointer will be provided if `path.is_none()`."] # [cfg (any (all (apple_targets , feature = "mount") , all (linux_android , any (feature = "mount" , feature = "fanotify"))))] pub (crate) fn with_opt_nix_path < P , T , F > (path : Option < & P > , f : F) -> Result < T > where P : ? Sized + NixPath , F : FnOnce (* const libc :: c_char) -> T , { match path { Some (path) => path . with_nix_path (| p_str | f (p_str . as_ptr ())) , None => Ok (f (ptr :: null ())) , } }
}