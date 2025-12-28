use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Creates an iterator that endlessly repeats a single element."] pub fn repeat < T : Clone , E > (value : T) -> Repeat < T , E > { Repeat (value , PhantomData) }