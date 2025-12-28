use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L : Iterator < Item = T > , R : Iterator < Item = T > , T , U : PartialEq , F : Fn (& T) -> U > DifferenceIter < L , R , F > { fn new (left : L , right : R , compare : F) -> Self { Self { left : left . fuse () , right , compare , } } }