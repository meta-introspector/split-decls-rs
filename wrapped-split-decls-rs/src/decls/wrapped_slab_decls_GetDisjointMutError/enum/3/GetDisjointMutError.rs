use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , PartialEq , Eq)] # [doc = " The error type returned by [`Slab::get_disjoint_mut`]."] pub enum GetDisjointMutError { # [doc = " An index provided was not associated with a value."] IndexVacant , # [doc = " An index provided was out-of-bounds for the slab."] IndexOutOfBounds , # [doc = " Two indices provided were overlapping."] OverlappingIndices , }