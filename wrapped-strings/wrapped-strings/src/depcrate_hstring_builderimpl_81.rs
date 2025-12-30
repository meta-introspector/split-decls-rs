// Generated macro for impl_81 (impl)
macro_rules! Depcrate_hstring_builderimpl_81 {
() => {
// Module: crate::hstring_builder
// Provides: {"impl_81"}
// Dependencies: {}
impl From < HStringBuilder > for HSTRING { fn from (value : HStringBuilder) -> Self { if let Some (header) = value . as_header () { unsafe { header . data . offset (header . len as isize) . write (0) } ; let result = Self (value . 0) ; core :: mem :: forget (value) ; result } else { Self :: new () } } }
};
}
