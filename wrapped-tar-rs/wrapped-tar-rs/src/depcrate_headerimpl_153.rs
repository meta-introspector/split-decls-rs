// Generated macro for impl_153 (impl)
macro_rules! Depcrate_headerimpl_153 {
() => {
// Module: crate::header
// Provides: {"impl_153"}
// Dependencies: {}
impl fmt :: Debug for GnuHeader { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut f = f . debug_struct ("GnuHeader") ; self . as_header () . debug_fields (& mut f) ; if let Ok (atime) = self . atime () { f . field ("atime" , & atime) ; } if let Ok (ctime) = self . ctime () { f . field ("ctime" , & ctime) ; } f . field ("is_extended" , & self . is_extended ()) . field ("sparse" , & DebugSparseHeaders (& self . sparse)) . finish () } }
};
}
