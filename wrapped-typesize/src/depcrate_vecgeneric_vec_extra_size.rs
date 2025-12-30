// Generated macro for generic_vec_extra_size (function)
macro_rules! Depcrate_vecgeneric_vec_extra_size {
() => {
// Module: crate::vec
// Provides: {"generic_vec_extra_size"}
// Dependencies: {}
pub (crate) fn generic_vec_extra_size < 'a , T : TypeSize + 'a > (iter : impl Iterator < Item = & 'a T > , capacity : usize , len : usize ,) -> usize { iter . map (TypeSize :: get_size) . sum :: < usize > () + (capacity - len) * core :: mem :: size_of :: < T > () }
};
}
