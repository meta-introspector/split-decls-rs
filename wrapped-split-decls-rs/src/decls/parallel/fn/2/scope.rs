use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn scope < 'scope , OP , R > (op : OP) -> R where OP : FnOnce (& rustc_thread_pool :: Scope < 'scope >) -> R + DynSend , R : DynSend , { let op = FromDyn :: from (op) ; rustc_thread_pool :: scope (| s | FromDyn :: from (op . into_inner () (s))) . into_inner () }
}