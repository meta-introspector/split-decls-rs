// Generated macro for impl_36 (impl)
macro_rules! Depcrate_serimpl_36 {
() => {
// Module: crate::ser
// Provides: {"impl_36"}
// Dependencies: {}
impl < S > Serializer < S > { # [doc = " Build a serializer adapter with reasonable default `red_zone` (64 KB)"] # [doc = " and `stack_size` (2 MB)."] pub fn new (serializer : S) -> Self { let default_param = Param :: default () ; Serializer { ser : serializer , red_zone : default_param . red_zone , stack_size : default_param . stack_size , } } }
};
}
