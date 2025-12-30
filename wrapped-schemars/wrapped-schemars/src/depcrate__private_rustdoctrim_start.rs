// Generated macro for trim_start (function)
macro_rules! Depcrate__private_rustdoctrim_start {
() => {
// Module: crate::_private::rustdoc
// Provides: {"trim_start"}
// Dependencies: {}
const fn trim_start (mut bytes : & [u8] , chr : u8) -> & [u8] { while let [first , rest @ ..] = bytes { if * first == chr { bytes = rest ; } else { break ; } } bytes }
};
}
