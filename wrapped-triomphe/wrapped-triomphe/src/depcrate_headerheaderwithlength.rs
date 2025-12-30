// Generated macro for HeaderWithLength (struct)
macro_rules! Depcrate_headerHeaderWithLength {
() => {
// Module: crate::header
// Provides: {"HeaderWithLength"}
// Dependencies: {}
# [doc = " Header data with an inline length. Consumers that use HeaderWithLength as the"] # [doc = " Header type in HeaderSlice can take advantage of ThinArc."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash)] # [repr (C)] pub struct HeaderWithLength < H > { # [doc = " The fixed-sized data."] pub header : H , # [doc = " The slice length."] pub length : usize , }
};
}
