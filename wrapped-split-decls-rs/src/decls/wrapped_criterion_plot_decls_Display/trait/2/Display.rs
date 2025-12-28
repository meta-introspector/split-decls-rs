use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Enums that can produce gnuplot code"] trait Display < S > { # [doc = " Translates the enum in gnuplot code"] fn display (& self) -> S ; }
}