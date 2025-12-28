use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Alias for the output size of [`OutputSizeUser`] implementors."] pub type OutputSize < T > = < T as OutputSizeUser > :: OutputSize ;