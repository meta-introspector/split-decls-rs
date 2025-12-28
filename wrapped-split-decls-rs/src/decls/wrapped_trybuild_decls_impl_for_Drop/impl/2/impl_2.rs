use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [doc (hidden)] impl Drop for TestCases { fn drop (& mut self) { if ! thread :: panicking () { self . runner . borrow_mut () . run () ; } } }
}