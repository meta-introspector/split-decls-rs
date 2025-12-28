use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Types which return data with the given size."] pub trait OutputSizeUser { # [doc = " Size of the output in bytes."] type OutputSize : ArraySize ; # [doc = " Return output size in bytes."] # [inline (always)] fn output_size () -> usize { Self :: OutputSize :: USIZE } }