use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FileSystemOracle { pub fn new () -> Self { Self } pub fn validate_path (& self , path : & Path) -> IoResult < () > { let path_str = path . to_string_lossy () ; if path_str . contains ("..") { return Err (IoError :: new (std :: io :: ErrorKind :: PermissionDenied , "Path traversal detected")) ; } let forbidden_paths = ["/etc/passwd" , "/etc/shadow" , "/root"] ; for forbidden in & forbidden_paths { if path_str . starts_with (forbidden) { return Err (IoError :: new (std :: io :: ErrorKind :: PermissionDenied , "Access to sensitive path denied")) ; } } Ok (()) } }
}