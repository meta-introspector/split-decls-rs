use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum AllocatorTy { Layout , Ptr , ResultPtr , Unit , Usize , }