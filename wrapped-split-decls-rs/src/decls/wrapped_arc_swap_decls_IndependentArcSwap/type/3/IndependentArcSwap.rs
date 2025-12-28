use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " An atomic storage that doesn't share the internal generation locks with others."] # [doc = ""] # [doc = " This makes it bigger and it also might suffer contention (on the HW level) if used from many"] # [doc = " threads at once. On the other hand, it can't block writes in other instances."] # [doc = ""] # [doc = " See the [`IndependentStrategy`] for further details."] # [doc (hidden)] pub type IndependentArcSwap < T > = ArcSwapAny < Arc < T > , IndependentStrategy > ;
}