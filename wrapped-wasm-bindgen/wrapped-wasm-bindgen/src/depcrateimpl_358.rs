// Generated macro for impl_358 (impl)
macro_rules! Depcrateimpl_358 {
() => {
// Module: crate
// Provides: {"impl_358"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E > From < E > for JsError where E : std :: error :: Error , { fn from (error : E) -> Self { use std :: string :: ToString ; JsError :: new (& error . to_string ()) } }
};
}
