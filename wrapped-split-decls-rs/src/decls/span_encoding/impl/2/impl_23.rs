use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl SpanInterner { fn intern (& mut self , span_data : & SpanData) -> u32 { let (index , _) = self . spans . insert_full (* span_data) ; index as u32 } }
}