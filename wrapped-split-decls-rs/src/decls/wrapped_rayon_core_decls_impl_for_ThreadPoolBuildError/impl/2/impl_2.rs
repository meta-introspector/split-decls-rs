use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ThreadPoolBuildError { fn new (kind : ErrorKind) -> ThreadPoolBuildError { ThreadPoolBuildError { kind } } fn is_unsupported (& self) -> bool { matches ! (& self . kind , ErrorKind :: IOError (e) if e . kind () == io :: ErrorKind :: Unsupported) } }
}