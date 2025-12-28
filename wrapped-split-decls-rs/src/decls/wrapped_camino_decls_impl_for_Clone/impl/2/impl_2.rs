use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Clone for Box < Utf8Path > { fn clone (& self) -> Self { let boxed : Box < Path > = self . 0 . into () ; let ptr = Box :: into_raw (boxed) as * mut Utf8Path ; unsafe { Box :: from_raw (ptr) } } }
}