use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl OutOfRange { const fn new () -> OutOfRange { OutOfRange { _private : () } } }
}