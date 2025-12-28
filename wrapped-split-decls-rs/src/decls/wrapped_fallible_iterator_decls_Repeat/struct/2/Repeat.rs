use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An iterator that endlessly repeats a single element."] # [derive (Clone , Debug)] pub struct Repeat < T : Clone , E > (T , PhantomData < E >) ;
}