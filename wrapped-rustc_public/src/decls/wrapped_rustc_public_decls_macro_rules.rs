use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! bridge_impl {
    ($name:ident, $ty:ty) => {
        impl rustc_public_bridge::bridge::$name < compiler_interface::BridgeTys > for $ty
        { fn new(def : crate ::DefId) -> Self { Self(def) } }
    };
}
