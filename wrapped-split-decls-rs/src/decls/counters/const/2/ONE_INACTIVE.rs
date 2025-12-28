use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Constant that can be added to add one inactive thread."] # [doc = " An inactive thread is either idle, sleepy, or sleeping."] const ONE_INACTIVE : usize = 1 << INACTIVE_SHIFT ;
}