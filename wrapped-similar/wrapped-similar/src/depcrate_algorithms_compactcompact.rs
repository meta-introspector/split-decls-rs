// Generated macro for Compact (struct)
macro_rules! Depcrate_algorithms_compactCompact {
() => {
// Module: crate::algorithms::compact
// Provides: {"Compact"}
// Dependencies: {}
# [doc = " Performs semantic cleanup operations on a diff."] # [doc = ""] # [doc = " This merges similar ops together but also tries to move hunks up and"] # [doc = " down the diff with the desire to connect as many hunks as possible."] # [doc = " It still needs to be combined with [`Replace`](crate::algorithms::Replace)"] # [doc = " to get actual replace diff ops out."] # [derive (Debug)] pub struct Compact < 'old , 'new , Old : ? Sized , New : ? Sized , D > { d : D , ops : Vec < DiffOp > , old : & 'old Old , new : & 'new New , }
};
}
