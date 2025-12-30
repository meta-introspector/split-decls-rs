// Generated macro for impl_685 (impl)
macro_rules! Depcrate_ffi_os_strimpl_685 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_685"}
// Dependencies: {}
# [unstable (feature = "slice_concat_ext" , issue = "27747")] impl < S : Borrow < OsStr > > alloc :: slice :: Join < & OsStr > for [S] { type Output = OsString ; fn join (slice : & Self , sep : & OsStr) -> OsString { let Some ((first , suffix)) = slice . split_first () else { return OsString :: new () ; } ; let first_owned = first . borrow () . to_owned () ; suffix . iter () . fold (first_owned , | mut a , b | { a . push (sep) ; a . push (b . borrow ()) ; a }) } }
};
}
