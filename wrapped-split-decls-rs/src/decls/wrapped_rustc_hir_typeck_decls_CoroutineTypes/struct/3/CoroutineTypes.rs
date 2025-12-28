use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " When `check_fn` is invoked on a coroutine (i.e., a body that"] # [doc = " includes yield), it returns back some information about the yield"] # [doc = " points."] # [derive (Debug , PartialEq , Copy , Clone)] struct CoroutineTypes < 'tcx > { # [doc = " Type of coroutine argument / values returned by `yield`."] resume_ty : Ty < 'tcx > , # [doc = " Type of value that is yielded."] yield_ty : Ty < 'tcx > , }
}