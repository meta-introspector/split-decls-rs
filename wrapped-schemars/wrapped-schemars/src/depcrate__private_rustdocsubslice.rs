// Generated macro for subslice (function)
macro_rules! Depcrate__private_rustdocsubslice {
() => {
// Module: crate::_private::rustdoc
// Provides: {"subslice"}
// Dependencies: {}
const fn subslice (mut bytes : & [u8] , mut start : usize , end : usize) -> & [u8] { let mut trim_end_count = bytes . len () - end ; if trim_end_count > 0 { while let [rest @ .. , _last] = bytes { bytes = rest ; trim_end_count -= 1 ; if trim_end_count == 0 { break ; } } } if start > 0 { while let [_first , rest @ ..] = bytes { bytes = rest ; start -= 1 ; if start == 0 { break ; } } } bytes }
};
}
