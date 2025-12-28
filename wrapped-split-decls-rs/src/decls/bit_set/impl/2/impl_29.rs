use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : Idx > BitIter < 'a , T > { # [inline] fn new (words : & 'a [Word]) -> BitIter < 'a , T > { BitIter { word : 0 , offset : usize :: MAX - (WORD_BITS - 1) , iter : words . iter () , marker : PhantomData , } } }