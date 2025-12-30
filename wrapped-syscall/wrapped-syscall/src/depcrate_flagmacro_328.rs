// Generated macro for macro_328 (macro)
macro_rules! Depcrate_flagmacro_328 {
() => {
// Module: crate::flag
// Provides: {"macro_328"}
// Dependencies: {}
bitflags ! { pub struct MremapFlags : usize { const FIXED = 1 ; const FIXED_REPLACE = 3 ; # [doc = " Alias's memory region at `old_address` to `new_address` such that both regions share"] # [doc = " the same frames."] const KEEP_OLD = 1 << 2 ; } }
};
}
