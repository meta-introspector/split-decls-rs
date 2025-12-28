use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Failure variant with collected diagnostics."] pub struct ErrParse < T > { pub failures : Vec < Failure > , _phantom : std :: marker :: PhantomData < T > , }