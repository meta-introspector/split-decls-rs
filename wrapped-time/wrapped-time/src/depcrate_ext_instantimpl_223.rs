// Generated macro for impl_223 (impl)
macro_rules! Depcrate_ext_instantimpl_223 {
() => {
// Module: crate::ext::instant
// Provides: {"impl_223"}
// Dependencies: {}
impl InstantExt for StdInstant { # [inline] fn checked_add_signed (& self , duration : Duration) -> Option < Self > { if duration . is_positive () { self . checked_add (duration . unsigned_abs ()) } else if duration . is_negative () { self . checked_sub (duration . unsigned_abs ()) } else { debug_assert ! (duration . is_zero ()) ; Some (* self) } } # [inline] fn checked_sub_signed (& self , duration : Duration) -> Option < Self > { if duration . is_positive () { self . checked_sub (duration . unsigned_abs ()) } else if duration . is_negative () { self . checked_add (duration . unsigned_abs ()) } else { debug_assert ! (duration . is_zero ()) ; Some (* self) } } # [inline] fn signed_duration_since (& self , earlier : Self) -> Duration { if * self > earlier { self . saturating_duration_since (earlier) . try_into () . unwrap_or (Duration :: MAX) } else { earlier . saturating_duration_since (* self) . try_into () . map_or (Duration :: MIN , | d : Duration | - d) } } }
};
}
