use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , N : ArrayLength > DerefMut for GenericArray < T , N > { # [inline (always)] fn deref_mut (& mut self) -> & mut [T] { GenericArray :: as_mut_slice (self) } }