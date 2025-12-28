use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ThreadInfo { fn new (stealer : Stealer < JobRef >) -> ThreadInfo { ThreadInfo { primed : LockLatch :: new () , stopped : LockLatch :: new () , terminate : OnceLatch :: new () , stealer , } } }
}