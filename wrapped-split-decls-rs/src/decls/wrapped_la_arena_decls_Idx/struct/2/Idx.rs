use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The index of a value allocated in an arena that holds `T`s."] pub struct Idx < T > { raw : RawIdx , _ty : PhantomData < fn () -> T > , }
}