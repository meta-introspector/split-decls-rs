// Generated macro for impl_2799 (impl)
macro_rules! Depcrate_pathimpl_2799 {
() => {
// Module: crate::path
// Provides: {"impl_2799"}
// Dependencies: {}
impl < 'a > Prefix < 'a > { # [inline] fn len (& self) -> usize { use self :: Prefix :: * ; fn os_str_len (s : & OsStr) -> usize { s . as_encoded_bytes () . len () } match * self { Verbatim (x) => 4 + os_str_len (x) , VerbatimUNC (x , y) => { 8 + os_str_len (x) + if os_str_len (y) > 0 { 1 + os_str_len (y) } else { 0 } } VerbatimDisk (_) => 6 , UNC (x , y) => 2 + os_str_len (x) + if os_str_len (y) > 0 { 1 + os_str_len (y) } else { 0 } , DeviceNS (x) => 4 + os_str_len (x) , Disk (_) => 2 , } } # [doc = " Determines if the prefix is verbatim, i.e., begins with `\\\\?\\`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path::Prefix::*;"] # [doc = " use std::ffi::OsStr;"] # [doc = ""] # [doc = " assert!(Verbatim(OsStr::new(\"pictures\")).is_verbatim());"] # [doc = " assert!(VerbatimUNC(OsStr::new(\"server\"), OsStr::new(\"share\")).is_verbatim());"] # [doc = " assert!(VerbatimDisk(b'C').is_verbatim());"] # [doc = " assert!(!DeviceNS(OsStr::new(\"BrainInterface\")).is_verbatim());"] # [doc = " assert!(!UNC(OsStr::new(\"server\"), OsStr::new(\"share\")).is_verbatim());"] # [doc = " assert!(!Disk(b'C').is_verbatim());"] # [doc = " ```"] # [inline] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn is_verbatim (& self) -> bool { use self :: Prefix :: * ; matches ! (* self , Verbatim (_) | VerbatimDisk (_) | VerbatimUNC (..)) } # [inline] fn is_drive (& self) -> bool { matches ! (* self , Prefix :: Disk (_)) } # [inline] fn has_implicit_root (& self) -> bool { ! self . is_drive () } }
};
}
