use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn get_thread_id () -> u32 { std :: thread :: current () . id () . as_u64 () . get () as u32 }