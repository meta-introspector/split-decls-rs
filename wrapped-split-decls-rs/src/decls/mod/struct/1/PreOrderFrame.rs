use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct PreOrderFrame < Iter > { pre_order_idx : PreorderIndex , iter : Iter , }
}