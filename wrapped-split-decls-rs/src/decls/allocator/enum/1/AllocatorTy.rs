use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub enum AllocatorTy { Layout , Ptr , ResultPtr , Unit , Usize , }
}