use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Curve variant of Default"] trait CurveDefault < S > { # [doc = " Creates `curve::Properties` with default configuration"] fn default (s : S) -> Self ; }
}