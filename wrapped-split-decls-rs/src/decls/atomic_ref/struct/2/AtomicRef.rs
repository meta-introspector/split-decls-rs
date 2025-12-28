use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " This is essentially an `AtomicPtr` but is guaranteed to always be valid"] pub struct AtomicRef < T : 'static > (AtomicPtr < T > , PhantomData < & 'static T >) ;
}