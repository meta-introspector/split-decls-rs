use serde::{Deserialize, Serialize};
use std::collections::HashMap;

unsafe impl < N : ArrayLength > ArrayLength for UInt < N , B1 > where Self : IsWithinUsizeBound , { # [doc (hidden)] type ArrayType < T > = GenericArrayImplOdd < T , N :: ArrayType < T > > ; }