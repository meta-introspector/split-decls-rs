// Generated macro for into_overflowable_list (function)
macro_rules! Depcrate_overflowinto_overflowable_list {
() => {
// Module: crate::overflow
// Provides: {"into_overflowable_list"}
// Dependencies: {}
pub (crate) fn into_overflowable_list < 'a , T > (iter : impl Iterator < Item = & 'a T > ,) -> impl Iterator < Item = OverflowableItem < 'a > > where T : 'a + IntoOverflowableItem < 'a > , { iter . map (| x | IntoOverflowableItem :: into_overflowable_item (x)) }
};
}
