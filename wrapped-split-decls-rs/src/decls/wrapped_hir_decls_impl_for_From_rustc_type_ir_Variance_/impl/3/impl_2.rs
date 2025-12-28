use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < rustc_type_ir :: Variance > for Variance { # [inline] fn from (value : rustc_type_ir :: Variance) -> Self { match value { rustc_type_ir :: Variance :: Covariant => Variance :: Covariant , rustc_type_ir :: Variance :: Invariant => Variance :: Invariant , rustc_type_ir :: Variance :: Contravariant => Variance :: Contravariant , rustc_type_ir :: Variance :: Bivariant => Variance :: Bivariant , } } }