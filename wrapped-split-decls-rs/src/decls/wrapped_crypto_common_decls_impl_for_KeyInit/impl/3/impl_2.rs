use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > KeyInit for T where T : InnerInit , T :: Inner : KeyInit , { # [inline] fn new (key : & Key < Self >) -> Self { Self :: inner_init (T :: Inner :: new (key)) } # [inline] fn new_from_slice (key : & [u8]) -> Result < Self , InvalidLength > { T :: Inner :: new_from_slice (key) . map_err (| _ | InvalidLength) . map (Self :: inner_init) } # [inline] fn weak_key_test (key : & Key < Self >) -> Result < () , WeakKeyError > { T :: Inner :: weak_key_test (key) } }