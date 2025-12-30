// Generated macro for impl_401 (impl)
macro_rules! Depcrate_fs_raw_dirimpl_401 {
() => {
// Module: crate::fs::raw_dir
// Provides: {"impl_401"}
// Dependencies: {}
impl < 'a > fmt :: Debug for RawDirEntry < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("RawDirEntry") ; f . field ("file_name" , & self . file_name ()) ; f . field ("file_type" , & self . file_type ()) ; f . field ("ino" , & self . ino ()) ; f . field ("next_entry_cookie" , & self . next_entry_cookie ()) ; f . finish () } }
};
}
