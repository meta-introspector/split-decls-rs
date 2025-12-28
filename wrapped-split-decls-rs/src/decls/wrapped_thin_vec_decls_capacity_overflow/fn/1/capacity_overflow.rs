use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cold] fn capacity_overflow () -> ! { panic ! ("capacity overflow") }
}