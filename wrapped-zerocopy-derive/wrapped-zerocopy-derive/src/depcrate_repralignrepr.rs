// Generated macro for AlignRepr (enum)
macro_rules! Depcrate_reprAlignRepr {
() => {
// Module: crate::repr
// Provides: {"AlignRepr"}
// Dependencies: {}
# [doc = " `repr(packed(...))` or `repr(align(...))`"] # [cfg_attr (test , derive (Copy , Clone , Debug , Eq , PartialEq))] pub (crate) enum AlignRepr < Packed > { Packed (Packed) , Align (NonZeroU32) , }
};
}
