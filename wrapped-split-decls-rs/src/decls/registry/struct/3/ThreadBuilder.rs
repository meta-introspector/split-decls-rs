use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Thread builder used for customization via"] # [doc = " [`ThreadPoolBuilder::spawn_handler`](struct.ThreadPoolBuilder.html#method.spawn_handler)."] pub struct ThreadBuilder { name : Option < String > , stack_size : Option < usize > , worker : Worker < JobRef > , stealer : Stealer < JobRef > , registry : Arc < Registry > , index : usize , }
}