// Generated macro for impl_903 (impl)
macro_rules! Depcrate_io_buffered_bufreaderimpl_903 {
() => {
// Module: crate::io::buffered::bufreader
// Provides: {"impl_903"}
// Dependencies: {}
impl < T : ? Sized > SizeHint for BufReader < T > { # [inline] fn lower_bound (& self) -> usize { SizeHint :: lower_bound (self . get_ref ()) + self . buffer () . len () } # [inline] fn upper_bound (& self) -> Option < usize > { SizeHint :: upper_bound (self . get_ref ()) . and_then (| up | self . buffer () . len () . checked_add (up)) } }
};
}
