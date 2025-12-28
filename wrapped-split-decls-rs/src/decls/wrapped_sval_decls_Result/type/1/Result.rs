use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = "\nA generic streaming result.\n"] pub type Result < T = () , E = Error > = std :: result :: Result < T , E > ;