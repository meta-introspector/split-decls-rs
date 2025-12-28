use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator that yields nothing."] # [derive (Clone , Debug)] pub struct Empty < T , E > (PhantomData < T > , PhantomData < E >) ;
}