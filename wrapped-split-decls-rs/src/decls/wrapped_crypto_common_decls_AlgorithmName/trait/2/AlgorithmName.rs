use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Trait which stores algorithm name constant, used in `Debug` implementations."] pub trait AlgorithmName { # [doc = " Write algorithm name into `f`."] fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; }
}