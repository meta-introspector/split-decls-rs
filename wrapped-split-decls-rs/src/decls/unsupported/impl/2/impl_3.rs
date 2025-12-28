use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Lock { pub fn new (_p : & Path , _wait : bool , _create : bool , _exclusive : bool) -> io :: Result < Lock > { let msg = "file locks not supported on this platform" ; Err (io :: Error :: new (io :: ErrorKind :: Other , msg)) } pub fn error_unsupported (_err : & io :: Error) -> bool { true } }
}