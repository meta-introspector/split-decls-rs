// Generated macro for Split (struct)
macro_rules! Depcrate_ioSplit {
() => {
// Module: crate::io
// Provides: {"Split"}
// Dependencies: {}
# [doc = " An iterator over the contents of an instance of `BufRead` split on a"] # [doc = " particular byte."] # [doc = ""] # [doc = " This struct is generally created by calling [`split`] on a `BufRead`."] # [doc = " Please see the documentation of [`split`] for more details."] # [doc = ""] # [doc = " [`split`]: BufRead::split"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct Split < B > { buf : B , delim : u8 , }
};
}
