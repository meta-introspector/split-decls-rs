// Generated macro for strchr (function)
macro_rules! Depcrate__private_rustdocstrchr {
() => {
// Module: crate::_private::rustdoc
// Provides: {"strchr"}
// Dependencies: {}
const fn strchr (bytes : & [u8] , chr : u8) -> Option < usize > { let len = bytes . len () ; let mut i = 0 ; while i < len { if bytes [i] == chr { return Some (i) ; } i += 1 ; } None }
};
}
