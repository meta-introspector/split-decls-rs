// Generated macro for Lines (struct)
macro_rules! Depcrate_ioLines {
() => {
// Module: crate::io
// Provides: {"Lines"}
// Dependencies: {}
# [doc = " An iterator over the lines of an instance of `BufRead`."] # [doc = ""] # [doc = " This struct is generally created by calling [`lines`] on a `BufRead`."] # [doc = " Please see the documentation of [`lines`] for more details."] # [doc = ""] # [doc = " [`lines`]: BufRead::lines"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] # [cfg_attr (not (test) , rustc_diagnostic_item = "IoLines")] pub struct Lines < B > { buf : B , }
};
}
