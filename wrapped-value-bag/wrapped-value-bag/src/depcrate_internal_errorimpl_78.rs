// Generated macro for impl_78 (impl)
macro_rules! Depcrate_internal_errorimpl_78 {
() => {
// Module: crate::internal::error
// Provides: {"impl_78"}
// Dependencies: {}
impl < 's , 'f > Slot < 's , 'f > { # [doc = " Fill the slot with an error."] # [doc = ""] # [doc = " The given value doesn't need to satisfy any particular lifetime constraints."] pub fn fill_error < T > (self , value : T) -> Result < () , crate :: Error > where T : error :: Error + 'static , { self . fill (| visitor | visitor . error (& value)) } # [doc = " Fill the slot with an error."] pub fn fill_dyn_error (self , value : & (dyn error :: Error + 'static)) -> Result < () , crate :: Error > { self . fill (| visitor | visitor . error (value)) } }
};
}
