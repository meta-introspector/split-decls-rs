// Generated macro for macro_if (macro)
macro_rules! Depcrate_macro_ifmacro_if {
() => {
// Module: crate::macro_if
// Provides: {"macro_if"}
// Dependencies: {}
# [macro_export] macro_rules ! macro_if { (true => $ ($ t : tt) *) => { $ ($ t) * } ; (false => $ ($ t : tt) *) => { } ; (if true { $ ($ t : tt) * } else { $ ($ f : tt) * }) => { $ ($ t) * } ; (if false { $ ($ t : tt) * } else { $ ($ f : tt) * }) => { $ ($ f) * } ; (if0 0 { $ ($ t : tt) * } else { $ ($ f : tt) * }) => { $ ($ t) * } ; (if0 $ n : literal { $ ($ t : tt) * } else { $ ($ f : tt) * }) => { $ ($ f) * } ; (iftt () { $ ($ t : tt) * } else { $ ($ f : tt) * }) => { $ ($ f) * } ; (iftt ($ ($ tt : tt) +) { $ ($ t : tt) * } else { $ ($ f : tt) * }) => { $ ($ t) * } ; }
};
}
