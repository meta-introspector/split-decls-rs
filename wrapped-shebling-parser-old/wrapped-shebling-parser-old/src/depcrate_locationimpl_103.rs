// Generated macro for impl_103 (impl)
macro_rules! Depcrate_locationimpl_103 {
() => {
// Module: crate::location
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a , S > From < S > for Location where S : Borrow < Span < 'a > > , { fn from (value : S) -> Self { let value = value . borrow () ; Self { offset : value . location_offset () , line : value . location_line () , column : value . get_utf8_column () , } } }
};
}
