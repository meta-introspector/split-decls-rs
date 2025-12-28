use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BitIter < 'a , T : Idx > { # [doc = " A copy of the current word, but with any already-visited bits cleared."] # [doc = " (This lets us use `trailing_zeros()` to find the next set bit.) When it"] # [doc = " is reduced to 0, we move onto the next word."] word : Word , # [doc = " The offset (measured in bits) of the current word."] offset : usize , # [doc = " Underlying iterator over the words."] iter : slice :: Iter < 'a , Word > , marker : PhantomData < T > , }