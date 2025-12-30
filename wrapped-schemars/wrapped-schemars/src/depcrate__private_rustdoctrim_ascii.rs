// Generated macro for trim_ascii (function)
macro_rules! Depcrate__private_rustdoctrim_ascii {
() => {
// Module: crate::_private::rustdoc
// Provides: {"trim_ascii"}
// Dependencies: {}
const fn trim_ascii (mut bytes : & [u8]) -> & [u8] { while let [first , rest @ ..] = bytes { if first . is_ascii_whitespace () { bytes = rest ; } else { break ; } } while let [rest @ .. , last] = bytes { if last . is_ascii_whitespace () { bytes = rest ; } else { break ; } } bytes }
};
}
