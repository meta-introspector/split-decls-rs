use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A specialized [`Result`] type that provides Windows error information."] pub type Result < T > = core :: result :: Result < T , Error > ;