use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [allow (dead_code)] struct MemoryMap { sq_mmap : Mmap , sqe_mmap : Mmap , cq_mmap : Option < Mmap > , }
}