use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Trait for hash functions with customization string for domain separation."] pub trait CustomizedInit : Sized { # [doc = " Create new hasher instance with the given customization string."] fn new_customized (customization : & [u8]) -> Self ; }