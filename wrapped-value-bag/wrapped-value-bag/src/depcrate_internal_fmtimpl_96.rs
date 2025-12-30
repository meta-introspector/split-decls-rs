// Generated macro for impl_96 (impl)
macro_rules! Depcrate_internal_fmtimpl_96 {
() => {
// Module: crate::internal::fmt
// Provides: {"impl_96"}
// Dependencies: {}
impl < 's , 'f > Slot < 's , 'f > { # [doc = " Fill the slot with a debuggable value."] # [doc = ""] # [doc = " The given value doesn't need to satisfy any particular lifetime constraints."] pub fn fill_debug < T > (self , value : T) -> Result < () , Error > where T : Debug , { self . fill (| visitor | visitor . debug (& value)) } # [doc = " Fill the slot with a displayable value."] # [doc = ""] # [doc = " The given value doesn't need to satisfy any particular lifetime constraints."] pub fn fill_display < T > (self , value : T) -> Result < () , Error > where T : Display , { self . fill (| visitor | visitor . display (& value)) } }
};
}
