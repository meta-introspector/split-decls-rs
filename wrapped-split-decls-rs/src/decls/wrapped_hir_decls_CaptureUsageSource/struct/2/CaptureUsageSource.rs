use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct CaptureUsageSource { is_ref : bool , source : InFile < AstPtr < Either < ast :: Expr , ast :: Pat > > > , }
}