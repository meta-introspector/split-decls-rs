use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , N : ArrayLength > Deref for GenericArray < T , N > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & [T] { GenericArray :: as_slice (self) } }