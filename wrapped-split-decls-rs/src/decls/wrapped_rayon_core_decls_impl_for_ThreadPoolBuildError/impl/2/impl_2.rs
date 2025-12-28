use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ThreadPoolBuildError { fn new (kind : ErrorKind) -> ThreadPoolBuildError { ThreadPoolBuildError { kind } } fn is_unsupported (& self) -> bool { matches ! (& self . kind , ErrorKind :: IOError (e) if e . kind () == io :: ErrorKind :: Unsupported) } }