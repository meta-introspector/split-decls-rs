// Generated macro for impl_7 (impl)
macro_rules! Depcrate_deimpl_7 {
() => {
// Module: crate::de
// Provides: {"impl_7"}
// Dependencies: {}
impl < D > Deserializer < D > { # [doc = " Build a deserializer adapter with reasonable default `red_zone` (64 KB)"] # [doc = " and `stack_size` (2 MB)."] pub fn new (deserializer : D) -> Self { let default_param = Param :: default () ; Deserializer { de : deserializer , red_zone : default_param . red_zone , stack_size : default_param . stack_size , } } }
};
}
