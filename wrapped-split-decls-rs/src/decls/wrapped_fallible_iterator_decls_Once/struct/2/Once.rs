use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator that yields something exactly once."] # [derive (Clone , Debug)] pub struct Once < T , E > (Option < T > , PhantomData < E >) ;
}