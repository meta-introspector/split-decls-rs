use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > Idx < T > { # [doc = " Creates a new index from a [`RawIdx`]."] pub const fn from_raw (raw : RawIdx) -> Self { Idx { raw , _ty : PhantomData , } } # [doc = " Converts this index into the underlying [`RawIdx`]."] pub const fn into_raw (self) -> RawIdx { self . raw } }