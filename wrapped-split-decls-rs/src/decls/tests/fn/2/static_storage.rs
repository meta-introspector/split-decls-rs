use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn static_storage () { let slice = slice_owned (Box :: new (String :: from ("what")) , | _ | b"bytes boo") ; assert_eq ! (&* slice , b"bytes boo") ; }