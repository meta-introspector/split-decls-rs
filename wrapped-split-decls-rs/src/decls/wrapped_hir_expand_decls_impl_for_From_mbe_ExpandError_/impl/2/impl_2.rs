use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < mbe :: ExpandError > for ExpandError { fn from (mbe : mbe :: ExpandError) -> Self { ExpandError { inner : Arc :: new ((ExpandErrorKind :: Mbe (mbe . inner . 1 . clone ()) , mbe . inner . 0)) , } } }