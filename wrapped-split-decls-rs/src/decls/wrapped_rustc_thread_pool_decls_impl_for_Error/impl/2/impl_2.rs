use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Error for ThreadPoolBuildError { fn source (& self) -> Option < & (dyn Error + 'static) > { match & self . kind { ErrorKind :: GlobalPoolAlreadyInitialized => None , ErrorKind :: IOError (e) => Some (e) , } } }