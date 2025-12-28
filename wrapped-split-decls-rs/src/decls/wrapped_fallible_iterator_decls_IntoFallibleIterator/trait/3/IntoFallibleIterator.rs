use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Conversion into a `FallibleIterator`."] pub trait IntoFallibleIterator { # [doc = " The elements of the iterator."] type Item ; # [doc = " The error value of the iterator."] type Error ; # [doc = " The iterator."] type IntoFallibleIter : FallibleIterator < Item = Self :: Item , Error = Self :: Error > ; # [doc = " Creates a fallible iterator from a value."] fn into_fallible_iter (self) -> Self :: IntoFallibleIter ; }