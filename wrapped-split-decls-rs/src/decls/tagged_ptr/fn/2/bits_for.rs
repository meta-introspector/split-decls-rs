use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Returns the number of bits available for use for tags in a pointer to `T`"] # [doc = " (this is based on `T`'s alignment)."] pub const fn bits_for < T : ? Sized + Aligned > () -> u32 { crate :: aligned :: align_of :: < T > () . as_nonzero () . trailing_zeros () }