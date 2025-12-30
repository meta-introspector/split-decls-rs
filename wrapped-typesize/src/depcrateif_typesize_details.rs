// Generated macro for if_typesize_details (macro)
macro_rules! Depcrateif_typesize_details {
() => {
// Module: crate
// Provides: {"if_typesize_details"}
// Dependencies: {}
# [doc = " Passes through the given tokens if the `details` feature of `typesize` is enabled."] # [doc = ""] # [doc = " This is mainly useful for libaries making their own [`TypeSize`] to be compatible with `details` on or off."] # [macro_export] # [cfg (not (feature = "details"))] macro_rules ! if_typesize_details { ($ ($ tt : tt) *) => { } ; }
};
}
