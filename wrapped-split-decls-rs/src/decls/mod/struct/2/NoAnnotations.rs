use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The nil annotation accumulator, which does nothing."] struct NoAnnotations < S : Idx + Ord > (PhantomData < S >) ;
}