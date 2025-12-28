use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: normalize_bounds");
# [doc = " Normalizes a slice of bounds by replacing [`TraitBound::Slf`] with `slf`."] fn normalize_bounds (slf : Trait , bounds : & [TraitBound]) -> impl '_ + Iterator < Item = Trait > { bounds . iter () . map (move | bound | match bound { TraitBound :: Slf => slf , TraitBound :: Other (trt) => * trt , }) }
}